pub mod math_utils {
    /// Enum class that summarizes the sort order type.
    pub enum SortOrderType {
        ASCENDING,
        DESCENDING,
        UNSORTED
    }

    /// Return a list of divisors.
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate kirinnya_math_utility;
    /// use kirinnya_math_utility::math_utils;
    /// let divisor_list = math_utils::get_divisor_list(100, math_utils::SortOrderType::ASCENDING);
    /// //divisor_list == [1, 2, 4, 5, 10, 20, 25, 50, 100]
    /// ```
    pub fn get_divisor_list(num: i128, order_type: SortOrderType) -> Vec<i128> {
        let mut v = Vec::new();
        let mut i: i128 = 1;
        while i * i <= num {
            if num % i == 0 {
                v.push(i);
                if i != num / i {
                    v.push(num / i)
                }
            }
            i = i + 1;
        }
        match order_type {
            SortOrderType::ASCENDING => v.sort_by(|a, b| a.cmp(b)),
            SortOrderType::DESCENDING => v.sort_by(|a, b| b.cmp(a)),
            SortOrderType::UNSORTED => {},//don't sort
        }
        return v;
    }

}

#[cfg(test)]
mod tests {
  use super::*; // To access outside definitions.

  #[test]
  fn get_divisor_list_happy_path() {
    assert_eq!(math_utils::get_divisor_list(100, math_utils::SortOrderType::ASCENDING), vec![1, 2, 4, 5, 10, 20, 25, 50, 100]);
    assert_eq!(math_utils::get_divisor_list(100, math_utils::SortOrderType::DESCENDING), vec![100, 50, 25, 20, 10, 5, 4, 2, 1]);
    assert_eq!(math_utils::get_divisor_list(100, math_utils::SortOrderType::UNSORTED), vec![1, 100, 2, 50, 4, 25, 5, 20, 10]);
  }

  #[test]
  fn get_divisor_list_zero_value() {
    assert_eq!(math_utils::get_divisor_list(0, math_utils::SortOrderType::ASCENDING), vec![]);
  }

  #[test]
  fn get_divisor_list_negative_value() {
    assert_eq!(math_utils::get_divisor_list(-1, math_utils::SortOrderType::ASCENDING), vec![]);
  }
}