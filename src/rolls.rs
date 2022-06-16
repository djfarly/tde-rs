pub type Rolls = [i8; 3];
pub type Attributes = [i8; 3];
pub type RemainingSkillPoints = [i8; 3];

pub fn is_botch(&rolls: &Rolls) -> bool {
    (rolls[0] == 20 && rolls[1] == 20)
        || (rolls[0] == 20 && rolls[2] == 20)
        || (rolls[1] == 20 && rolls[2] == 20)
}

pub fn is_spectacular_botch(&rolls: &Rolls) -> bool {
    rolls[0] == 20 && rolls[1] == 20 && rolls[2] == 20
}

pub fn is_critical_success(&rolls: &Rolls) -> bool {
    (rolls[0] == 1 && rolls[1] == 1)
        || (rolls[0] == 1 && rolls[2] == 1)
        || (rolls[1] == 1 && rolls[2] == 1)
}

pub fn is_spectacular_success(&rolls: &Rolls) -> bool {
    rolls[0] == 1 && rolls[1] == 1 && rolls[2] == 1
}

pub fn get_quality_level(&remainder: &i8) -> i8 {
    if remainder >= 0 {
        let third: f64 = remainder as f64 / 3.;
        third.ceil().clamp(1., 6.) as i8
    } else {
        0
    }
}

pub fn get_cost_for_roll(&roll: &i8, &attribute: &i8, &modifier: &i8) -> i8 {
    if roll > (attribute as i8 + modifier) {
        (roll - (attribute as i8 + modifier)) as i8
    } else {
        0
    }
}

pub fn get_remaining_skill_points_fast(
    &rolls: &Rolls,
    &attributes: &Attributes,
    &skill_points: &i8,
    &modifier: &i8,
) -> i8 {
    skill_points
        - get_cost_for_roll(&rolls[0], &attributes[0], &modifier)
        - get_cost_for_roll(&rolls[1], &attributes[1], &modifier)
        - get_cost_for_roll(&rolls[2], &attributes[2], &modifier)
}

pub fn get_remaining_skill_points(
    &rolls: &Rolls,
    &attributes: &Attributes,
    &skill_points: &i8,
    &modifier: &i8,
) -> [i8; 3] {
    let remaining_0 = skill_points - get_cost_for_roll(&rolls[0], &attributes[0], &modifier);
    let remaining_1 = remaining_0 - get_cost_for_roll(&rolls[1], &attributes[1], &modifier);
    let remaining_2 = remaining_1 - get_cost_for_roll(&rolls[2], &attributes[2], &modifier);
    [remaining_0, remaining_1, remaining_2]
}

#[derive(Debug)]
pub enum SkillCheckResult {
    SpectacularBotch,
    Botch,
    Fail {
        remaining_skill_points: RemainingSkillPoints,
    },
    Success {
        quality_level: i8,
        remaining_skill_points: RemainingSkillPoints,
    },
    CriticalSuccess {
        quality_level: i8,
        remaining_skill_points: RemainingSkillPoints,
    },
    SpectacularSuccess {
        quality_level: i8,
        remaining_skill_points: RemainingSkillPoints,
    },
}

pub fn skill_check(
    &rolls: &Rolls,
    &attributes: &Attributes,
    &skill_points: &i8,
    &modifier: &i8,
) -> SkillCheckResult {
    if is_spectacular_botch(&rolls) {
        SkillCheckResult::SpectacularBotch
    } else if is_botch(&rolls) {
        SkillCheckResult::Botch
    } else if is_critical_success(&rolls) {
        SkillCheckResult::CriticalSuccess {
            quality_level: get_quality_level(&skill_points),
            remaining_skill_points: [skill_points, skill_points, skill_points],
        }
    } else if is_spectacular_success(&rolls) {
        SkillCheckResult::SpectacularSuccess {
            quality_level: get_quality_level(&skill_points),
            remaining_skill_points: [skill_points, skill_points, skill_points],
        }
    } else {
        let remaining_skill_points =
            get_remaining_skill_points(&rolls, &attributes, &skill_points, &modifier);
        let quality_level = get_quality_level(&remaining_skill_points[2]);
        if quality_level > 0 {
            SkillCheckResult::Success {
                quality_level,
                remaining_skill_points: remaining_skill_points,
            }
        } else {
            SkillCheckResult::Fail {
                remaining_skill_points: remaining_skill_points,
            }
        }
    }
}

pub fn skill_check_fast(
    &rolls: &Rolls,
    &attributes: &Attributes,
    &skill_points: &i8,
    &modifier: &i8,
) -> i8 {
    if is_spectacular_botch(&rolls) || is_botch(&rolls) {
        0
    } else if is_critical_success(&rolls) || is_spectacular_success(&rolls) {
        get_quality_level(&skill_points)
    } else {
        let remainder =
            get_remaining_skill_points_fast(&rolls, &attributes, &skill_points, &modifier);
        get_quality_level(&remainder)
    }
}

#[derive(Debug)]
pub struct Chance {
    success: f64,
    quality: [f64; 6],
}

pub fn calc_chance(&attributes: &Attributes, &skill_points: &i8, &modifier: &i8) -> Chance {
    let mut successes: u16 = 0;
    let mut quality_level_successes: [u16; 6] = [0, 0, 0, 0, 0, 0];
    for d1 in 1_i8..21_i8 {
        for d2 in 1_i8..21_i8 {
            for d3 in 1_i8..21_i8 {
                let quality_level =
                    skill_check_fast(&[d1, d2, d3], &attributes, &skill_points, &modifier);

                if quality_level > 1 {
                    successes += 1;
                    quality_level_successes[(quality_level - 1) as usize] += 1;
                }
            }
        }
    }
    Chance {
        success: successes as f64 / 8000.,
        quality: quality_level_successes.map(|successes| successes as f64 / 8000.),
    }
}
