pub mod min_operations;

#[cfg(test)]
mod tests {

    use super::*;
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test_log::test]
    fn min_operations_test() {
        use min_operations::min_operations;
        init();

        info!("=== min_operations test ===");

        let nums1 = vec![1, 2, 3, 4, 5, 6];
        let nums2 = vec![1, 1, 2, 2, 2, 2];

        info!("{:?} {:?}: ", nums1, nums2,);
        assert_eq!(min_operations(nums1.clone(), nums2.clone()), 3);

        let nums1 = vec![1, 1, 1, 1, 1, 1, 1];
        let nums2 = vec![6];

        info!("{:?} {:?}: ", nums1, nums2,);
        assert_eq!(min_operations(nums1.clone(), nums2.clone()), -1);

        let nums1 = vec![6, 6];
        let nums2 = vec![1];

        info!("{:?} {:?}: ", nums1, nums2,);
        assert_eq!(min_operations(nums1.clone(), nums2.clone()), 3);
    }

    #[test_log::test]
    fn min_operations_test_advanced() {
        use min_operations::min_operations;
        init();

        info!("=== min_operations_test_advanced test ===");

        let nums1 = vec![5, 6, 4, 3, 1, 2];
        let nums2 = vec![6, 3, 3, 1, 4, 5, 3, 4, 1, 3, 4];

        info!("{:?} {:?}: ", nums1, nums2,);
        assert_eq!(min_operations(nums1.clone(), nums2.clone()), 4);
    }

    #[test_log::test]
    fn min_operations_test_edge() {
        use min_operations::min_operations;
        init();

        info!("=== min_operations_test_edge test ===");

        let nums1 =
            vec![2, 2, 4, 3, 1, 1, 5, 2, 5, 2, 5, 6, 1, 1, 6, 4, 5, 2, 5, 3];
        let nums2 = vec![3, 3, 4];

        info!("{:?} {:?}: ", nums1, nums2,);
        assert_eq!(min_operations(nums1.clone(), nums2.clone()), -1);
    }

    // #[test_log::test]
    // fn min_operations_test_long() {
    //     use min_operations::min_operations;
    //     init();

    //     info!("=== min_operations_test_long test ===");

    //     let nums1 = vec![
    //         1, 5, 5, 2, 1, 1, 1, 1, 4, 4, 4, 1, 5, 2, 2, 4, 6, 5, 1, 5, 3, 5,
    //         6, 2, 3, 1, 5, 4, 4, 1, 2, 4, 1, 1, 6, 3, 6, 4, 4, 4, 3, 5, 5, 5,
    //         2, 6, 4, 2, 5, 4, 2, 6, 3, 4, 6, 1, 5, 3, 2, 3, 5, 2, 1, 3, 2, 4,
    //         4, 4, 5, 3, 5, 5, 4, 1, 1, 6, 5, 6, 3, 5, 3, 6, 5, 6, 5, 4, 4, 4,
    //         5, 6, 6, 6, 4, 2, 4, 6, 1, 2, 1, 5, 3, 4, 5, 5, 6, 6, 1, 4, 3, 1,
    //         5, 3, 4, 1, 2, 1, 4, 4, 5, 6, 5, 3, 1, 5, 1, 3, 3, 6, 5, 3, 5, 6,
    //         2, 6, 3, 1, 2, 3, 3, 1, 1, 4, 3, 2, 6, 6, 2, 1, 2, 4, 3, 5, 5, 4,
    //         3, 1, 1, 5, 2, 5, 1, 4, 5, 6, 4, 5, 2, 1, 2, 5, 3, 2, 6, 3, 4, 3,
    //         4, 5, 4, 6, 3, 4, 4, 3, 3, 4, 2, 2, 6, 2, 6, 3, 1, 1, 5, 3, 1, 1,
    //         4, 2, 5, 5, 5, 4, 3, 6, 5, 5, 5, 1, 1, 3, 6, 2, 3, 6, 3, 4, 2, 5,
    //         4, 4, 3, 5, 6, 4, 3, 5, 1, 1, 3, 3, 1, 1, 6, 4, 6, 2, 1, 4, 3, 5,
    //         5,
    //     ];
    //     let nums2 = vec![
    //         1, 2, 5, 4, 3, 3, 5, 1, 1, 6, 2, 5, 4, 4, 5, 6, 6, 4, 2, 5, 6, 2,
    //         3, 4, 5, 2, 4, 4, 3, 6, 6, 5, 4, 1, 2, 1, 2, 3, 3, 2, 6, 1, 1, 1,
    //         1, 3, 5, 6, 2, 1, 1, 1, 4, 6, 5,
    //     ];

    //     info!("{:?} {:?}: ", nums1, nums2,);
    //     assert_eq!(min_operations(nums1.clone(), nums2.clone()), 184);
    // }
}
