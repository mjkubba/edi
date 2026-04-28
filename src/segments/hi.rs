use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct HI {
    pub hi01_health_care_code_information: String,
    pub hi02_health_care_code_information: String,
    pub hi03_health_care_code_information: String,
    pub hi04_health_care_code_information: String,
    pub hi05_health_care_code_information: String,
    pub hi06_health_care_code_information: String,
    pub hi07_health_care_code_information: String,
    pub hi08_health_care_code_information: String,
    pub hi09_health_care_code_information: String,
    pub hi10_health_care_code_information: String,
    pub hi11_health_care_code_information: String,
    pub hi12_health_care_code_information: String,
}

pub fn get_hi(hi_content: String) -> HI {
    let hi_parts: Vec<&str> = hi_content.split("*").collect();

    let mut hi = HI::default();

    if hi_parts.len() > 0 && !hi_parts[0].is_empty() {
        hi.hi01_health_care_code_information = get_element(&hi_parts, 0);
    }

    if hi_parts.len() > 1 && !hi_parts[1].is_empty() {
        hi.hi02_health_care_code_information = get_element(&hi_parts, 1);
    }

    if hi_parts.len() > 2 && !hi_parts[2].is_empty() {
        hi.hi03_health_care_code_information = get_element(&hi_parts, 2);
    }

    if hi_parts.len() > 3 && !hi_parts[3].is_empty() {
        hi.hi04_health_care_code_information = get_element(&hi_parts, 3);
    }

    if hi_parts.len() > 4 && !hi_parts[4].is_empty() {
        hi.hi05_health_care_code_information = get_element(&hi_parts, 4);
    }

    if hi_parts.len() > 5 && !hi_parts[5].is_empty() {
        hi.hi06_health_care_code_information = get_element(&hi_parts, 5);
    }

    if hi_parts.len() > 6 && !hi_parts[6].is_empty() {
        hi.hi07_health_care_code_information = get_element(&hi_parts, 6);
    }

    if hi_parts.len() > 7 && !hi_parts[7].is_empty() {
        hi.hi08_health_care_code_information = get_element(&hi_parts, 7);
    }

    if hi_parts.len() > 8 && !hi_parts[8].is_empty() {
        hi.hi09_health_care_code_information = get_element(&hi_parts, 8);
    }

    if hi_parts.len() > 9 && !hi_parts[9].is_empty() {
        hi.hi10_health_care_code_information = get_element(&hi_parts, 9);
    }

    if hi_parts.len() > 10 && !hi_parts[10].is_empty() {
        hi.hi11_health_care_code_information = get_element(&hi_parts, 10);
    }

    if hi_parts.len() > 11 && !hi_parts[11].is_empty() {
        hi.hi12_health_care_code_information = get_element(&hi_parts, 11);
    }

    hi
}

pub fn write_hi(hi: HI) -> String {
    if hi.hi01_health_care_code_information.is_empty() {
        return String::new();
    }

    let mut hi_content = String::new();
    hi_content.push_str("HI*");
    hi_content.push_str(&hi.hi01_health_care_code_information);

    if !hi.hi02_health_care_code_information.is_empty() {
        hi_content.push_str("*");
        hi_content.push_str(&hi.hi02_health_care_code_information);

        if !hi.hi03_health_care_code_information.is_empty() {
            hi_content.push_str("*");
            hi_content.push_str(&hi.hi03_health_care_code_information);

            if !hi.hi04_health_care_code_information.is_empty() {
                hi_content.push_str("*");
                hi_content.push_str(&hi.hi04_health_care_code_information);

                if !hi.hi05_health_care_code_information.is_empty() {
                    hi_content.push_str("*");
                    hi_content.push_str(&hi.hi05_health_care_code_information);

                    if !hi.hi06_health_care_code_information.is_empty() {
                        hi_content.push_str("*");
                        hi_content.push_str(&hi.hi06_health_care_code_information);

                        if !hi.hi07_health_care_code_information.is_empty() {
                            hi_content.push_str("*");
                            hi_content.push_str(&hi.hi07_health_care_code_information);

                            if !hi.hi08_health_care_code_information.is_empty() {
                                hi_content.push_str("*");
                                hi_content.push_str(&hi.hi08_health_care_code_information);

                                if !hi.hi09_health_care_code_information.is_empty() {
                                    hi_content.push_str("*");
                                    hi_content.push_str(&hi.hi09_health_care_code_information);

                                    if !hi.hi10_health_care_code_information.is_empty() {
                                        hi_content.push_str("*");
                                        hi_content.push_str(&hi.hi10_health_care_code_information);

                                        if !hi.hi11_health_care_code_information.is_empty() {
                                            hi_content.push_str("*");
                                            hi_content
                                                .push_str(&hi.hi11_health_care_code_information);

                                            if !hi.hi12_health_care_code_information.is_empty() {
                                                hi_content.push_str("*");
                                                hi_content.push_str(
                                                    &hi.hi12_health_care_code_information,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    hi_content.push_str("~");
    hi_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hi() {
        let hi_content = "BF*41090*D8*20050125".to_string();
        let hi = get_hi(hi_content);
        assert_eq!(hi.hi01_health_care_code_information, "BF");
        assert_eq!(hi.hi02_health_care_code_information, "41090");
        assert_eq!(hi.hi03_health_care_code_information, "D8");
        assert_eq!(hi.hi04_health_care_code_information, "20050125");
    }

    #[test]
    fn test_write_hi() {
        let hi = HI {
            hi01_health_care_code_information: "BF".to_string(),
            hi02_health_care_code_information: "41090".to_string(),
            hi03_health_care_code_information: "D8".to_string(),
            hi04_health_care_code_information: "20050125".to_string(),
            hi05_health_care_code_information: "".to_string(),
            hi06_health_care_code_information: "".to_string(),
            hi07_health_care_code_information: "".to_string(),
            hi08_health_care_code_information: "".to_string(),
            hi09_health_care_code_information: "".to_string(),
            hi10_health_care_code_information: "".to_string(),
            hi11_health_care_code_information: "".to_string(),
            hi12_health_care_code_information: "".to_string(),
        };

        let hi_content = write_hi(hi);
        assert_eq!(hi_content, "HI*BF*41090*D8*20050125~");
    }
}
