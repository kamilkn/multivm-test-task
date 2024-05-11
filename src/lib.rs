use rayon::prelude::*;
use std::sync::Arc;

pub fn apply_transformations(input: Vec<u64>, max_iterations: usize, threshold: usize) -> Vec<u64> {
    let transformation = move |num: u64| transform_number(num, max_iterations);
    parallel_process(input, threshold, transformation)
}

#[macro_export]
macro_rules! apply_transformations {
    ($input:expr) => {
        apply_transformations(
            $input,
            8,                   // Default maximum iterations
            num_cpus::get() * 2, // Default threshold based on CPU count
        )
    };
    ($input:expr, $max_iterations:expr, $threshold:expr) => {
        apply_transformations($input, $max_iterations, $threshold)
    };
}

fn transform_number(mut num: u64, max_iterations: usize) -> u64 {
    let mut count = 0;
    while num != 1 && count < max_iterations {
        num = if num % 2 == 0 { num / 2 } else { num * 3 + 1 };
        count += 1;
    }
    if num == 1 {
        count as u64
    } else {
        num
    }
}

fn parallel_process<T, R, F>(input: Vec<T>, threshold: usize, func: F) -> Vec<R>
where
    T: Send + Sync + 'static,
    R: Send + Sync + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    if input.len() < threshold {
        input.into_iter().map(&func).collect()
    } else {
        let func = Arc::new(func);
        input.into_par_iter().map(|item| (*func)(item)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_process() {
        let input = vec![1, 2, 3, 4, 5];
        let result = parallel_process(input, 3, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_transform_number() {
        assert_eq!(transform_number(1, 8), 0);
        assert_eq!(transform_number(2, 8), 1);
        assert_eq!(transform_number(3, 8), 7);
        assert_eq!(transform_number(100, 8), 88);
    }

    #[test]
    fn test_apply_transformations_empty_input() {
        let input = vec![];
        let max_iterations = 10;
        let threshold = 3;
        assert_eq!(
            apply_transformations(input, max_iterations, threshold),
            vec![]
        );
    }

    #[test]
    fn test_apply_transformations_single_element() {
        let input = vec![1];
        let max_iterations = 10;
        let threshold = 1;
        assert_eq!(
            apply_transformations(input, max_iterations, threshold),
            vec![0]
        );
    }

    #[test]
    fn test_apply_transformations_multiple_elements() {
        let input = vec![1, 2, 3];
        let max_iterations = 8;
        let threshold = 2;
        assert_eq!(
            apply_transformations(input, max_iterations, threshold),
            vec![0, 1, 7]
        );
    }

    #[test]
    fn test_apply_transformations_threshold_effect() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let max_iterations = 8;
        let threshold = 5;
        let expected_results = vec![0, 1, 7, 2, 5, 8, 40, 3, 52, 6];
        assert_eq!(
            apply_transformations(input, max_iterations, threshold),
            expected_results
        );
    }

    #[test]
    fn test_apply_transformations_macros() {
        let input = vec![1, 2, 3, 100];
        let result = apply_transformations!(input);
        assert_eq!(result, vec![0, 1, 7, 88]);
    }
}
