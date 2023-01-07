use crate::stack::Stack;

pub fn move_stack(
    source: &mut Stack,
    target: &mut Stack,
    helper_stack: &mut Stack,
    step: &mut u32,
    max_depth: Option<&usize>,
) -> Result<(), &'static str> {
    let depth = if max_depth.is_none() {
        source.rings.len()
    } else {
        max_depth.unwrap().clone()
    };
    if crate::IS_VERBOSE {
        println!("Move stack {}:{} -> {}", source.name, depth, target.name);
    }
    if source.rings.len() == 1 || depth == 1 {
        *step += 1;
        let move_result = source.move_ring(target);
        crate::display_stacks(&step, &vec![&source, &target, &helper_stack]);
        return move_result;
    }
    let next_depth = if max_depth.is_none() {
        source.rings.len() - 1
    } else {
        max_depth.unwrap().clone() - 1
    };
    let result = move_stack(source, helper_stack, target, step, Some(&next_depth));

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    *step += 1;
    if crate::IS_VERBOSE {
        println!("Move uncovered {} -> {}", source.name, target.name);
    }
    source.move_ring(target)?;
    crate::display_stacks(&step, &vec![&source, &target, &helper_stack]);
    if crate::IS_VERBOSE {
        println!(
            "Stack to uncovered {}:{}:{} -> {}",
            helper_stack.name, &depth, &next_depth, target.name
        );
    }
    return move_stack(helper_stack, target, source, step, Some(&next_depth));
}
