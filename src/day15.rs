#[cfg(test)]
mod solution {
    use ndarray::arr2;

    #[test]
    fn optimize_cookie_simple() {
        let properties = arr2(&[[-1i64, 2], [-2, 3], [6, -2], [3, -1]]);
        let butterscotch_teaspoons = |x| 100 - x;
        let mut highest_score = 0;
        for cinnamon_teaspoons in 0..100 {
            let x = arr2(&[
                [cinnamon_teaspoons],
                [butterscotch_teaspoons(cinnamon_teaspoons)],
            ]);
            let score = properties
                .dot(&x)
                .map(|property_score| {
                    if *property_score < 0 {
                        0
                    } else {
                        *property_score
                    }
                })
                .product();
            if score > highest_score {
                highest_score = score;
            }
        }

        assert_eq!(highest_score, 62842880);
    }

    /// Too difficult to generalize to n dimensions.
    #[test]
    fn optimize_cookie() {
        // Properties matrix
        let properties = arr2(&[
            [4i64, 0, -1, 0],
            [-2, 5, 0, 0],
            [0, -1, 5, -2],
            [0, 0, 0, 2],
        ]);

        let mut highest_score = 0;
        for frosting_teaspoons in 0..101 {
            for candy_teaspoons in 0..(101 - frosting_teaspoons) {
                for butterscotch_teaspoons in 0..(101 - frosting_teaspoons - candy_teaspoons) {
                    for sugar_teaspoons in
                        0..(101 - frosting_teaspoons - candy_teaspoons - butterscotch_teaspoons)
                    {
                        if frosting_teaspoons
                            + candy_teaspoons
                            + butterscotch_teaspoons
                            + sugar_teaspoons
                            == 100
                        {
                            let x = arr2(&[
                                [frosting_teaspoons],
                                [candy_teaspoons],
                                [butterscotch_teaspoons],
                                [sugar_teaspoons],
                            ]);
                            let score = properties
                                .dot(&x)
                                .map(|property_score| {
                                    if *property_score < 0 {
                                        0
                                    } else {
                                        *property_score
                                    }
                                })
                                .product();
                            if score > highest_score {
                                highest_score = score;
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(highest_score, 18965440);
    }

    #[test]
    fn optimize_cookie_with_calories() {
        // Properties matrix
        let properties = arr2(&[
            [4i64, 0, -1, 0],
            [-2, 5, 0, 0],
            [0, -1, 5, -2],
            [0, 0, 0, 2],
        ]);
        let calories = arr2(&[[5, 8, 6, 1]]);

        let mut highest_score = 0;
        for frosting_teaspoons in 0..101 {
            for candy_teaspoons in 0..(101 - frosting_teaspoons) {
                for butterscotch_teaspoons in 0..(101 - frosting_teaspoons - candy_teaspoons) {
                    for sugar_teaspoons in
                        0..(101 - frosting_teaspoons - candy_teaspoons - butterscotch_teaspoons)
                    {
                        if frosting_teaspoons
                            + candy_teaspoons
                            + butterscotch_teaspoons
                            + sugar_teaspoons
                            == 100
                        {
                            let x = arr2(&[
                                [frosting_teaspoons],
                                [candy_teaspoons],
                                [butterscotch_teaspoons],
                                [sugar_teaspoons],
                            ]);
                            if calories.dot(&x)[[0, 0]] != 500 {
                                continue;
                            }
                            let score = properties
                                .dot(&x)
                                .map(|property_score| {
                                    if *property_score < 0 {
                                        0
                                    } else {
                                        *property_score
                                    }
                                })
                                .product();
                            if score > highest_score {
                                highest_score = score;
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(highest_score, 15862900);
    }
}
