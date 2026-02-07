use crate::models::{Throw, Visit};
use crate::log_debug;


pub const CHECKOUT_TABLE_DOUBLE_OUT: [Option<&[(i32, i32)]>; 171] = [
    None,
    None,
    Some(&[(1, 2)]),
    Some(&[(1, 1), (1, 2)]),
    Some(&[(2, 2)]),
    Some(&[(1, 1), (2, 2)]),
    Some(&[(3, 2)]),
    Some(&[(3, 1), (2, 2)]),
    Some(&[(4, 2)]),
    Some(&[(1, 1), (4, 2)]),
    Some(&[(5, 2)]),
    Some(&[(3, 1), (4, 2)]),
    Some(&[(6, 2)]),
    Some(&[(5, 1), (4, 2)]),
    Some(&[(7, 2)]),
    Some(&[(7, 1), (4, 2)]),
    Some(&[(8, 2)]),
    Some(&[(1, 1), (8, 2)]),
    Some(&[(9, 2)]),
    Some(&[(3, 1), (8, 2)]),
    Some(&[(10, 2)]),
    Some(&[(5, 1), (8, 2)]),
    Some(&[(11, 2)]),
    Some(&[(7, 1), (8, 2)]),
    Some(&[(12, 2)]),
    Some(&[(9, 1), (8, 2)]),
    Some(&[(13, 2)]),
    Some(&[(11, 1), (8, 2)]),
    Some(&[(14, 2)]),
    Some(&[(13, 1), (8, 2)]),
    Some(&[(15, 2)]),
    Some(&[(15, 1), (8, 2)]),
    Some(&[(16, 2)]),
    Some(&[(1, 1), (16, 2)]),
    Some(&[(17, 2)]),
    Some(&[(3, 1), (16, 2)]),
    Some(&[(18, 2)]),
    Some(&[(5, 1), (16, 2)]),
    Some(&[(19, 2)]),
    Some(&[(7, 1), (16, 2)]),
    Some(&[(20, 2)]),
    Some(&[(9, 1), (16, 2)]),
    Some(&[(10, 1), (16, 2)]),
    Some(&[(11, 1), (16, 2)]),
    Some(&[(12, 1), (16, 2)]),
    Some(&[(13, 1), (16, 2)]),
    Some(&[(14, 1), (16, 2)]),
    Some(&[(15, 1), (16, 2)]),
    Some(&[(16, 1), (16, 2)]),
    Some(&[(17, 1), (16, 2)]),
    Some(&[(25, 2)]),
    Some(&[(19, 1), (16, 2)]),
    Some(&[(20, 1), (16, 2)]),
    Some(&[(13, 1), (20, 2)]),
    Some(&[(14, 1), (20, 2)]),
    Some(&[(15, 1), (20, 2)]),
    Some(&[(16, 1), (20, 2)]),
    Some(&[(17, 1), (20, 2)]),
    Some(&[(18, 1), (20, 2)]),
    Some(&[(19, 1), (20, 2)]),
    Some(&[(20, 1), (20, 2)]),
    Some(&[(15, 3), (8, 2)]),           // 61: T15, D8 (Two Dart Finish)
    Some(&[(10, 3), (16, 2)]),          // 62: T10, D16
    Some(&[(13, 3), (12, 2)]),          // 63: T13, D12
    Some(&[(16, 3), (8, 2)]),           // 64: T16, D8
    Some(&[(19, 3), (4, 2)]),           // 65: T19, D4
    Some(&[(10, 3), (18, 2)]),          // 66: T10, D18
    Some(&[(17, 3), (8, 2)]),           // 67: T17, D8
    Some(&[(20, 3), (4, 2)]),           // 68: T20, D4
    Some(&[(15, 3), (12, 2)]),          // 69: T15, D12
    Some(&[(10, 3), (20, 2)]),          // 70: T10, D20
    Some(&[(13, 3), (16, 2)]),          // 71: T13, D16
    Some(&[(16, 3), (12, 2)]),          // 72: T16, D12
    Some(&[(19, 3), (8, 2)]),           // 73: T19, D8
    Some(&[(14, 3), (16, 2)]),          // 74: T14, D16
    Some(&[(17, 3), (12, 2)]),          // 75: T17, D12
    Some(&[(20, 3), (8, 2)]),           // 76: T20, D8
    Some(&[(19, 3), (10, 2)]),          // 77: T19, D10
    Some(&[(18, 3), (12, 2)]),          // 78: T18, D12
    Some(&[(19, 3), (11, 2)]),          // 79: T19, D11
    Some(&[(20, 3), (10, 2)]),          // 80: T20, D10
    Some(&[(19, 3), (12, 2)]),          // 81: T19, D12
    Some(&[(14, 3), (20, 2)]),          // 82: T14, D20
    Some(&[(17, 3), (16, 2)]),          // 83: T17, D16
    Some(&[(20, 3), (12, 2)]),          // 84: T20, D12
    Some(&[(15, 3), (20, 2)]),          // 85: T15, D20
    Some(&[(18, 3), (16, 2)]),          // 86: T18, D16
    Some(&[(17, 3), (18, 2)]),          // 87: T17, D18
    Some(&[(16, 3), (20, 2)]),          // 88: T16, D20
    Some(&[(19, 3), (16, 2)]),          // 89: T19, D16
    Some(&[(20, 3), (15, 2)]),          // 90: T20, D15
    Some(&[(17, 3), (20, 2)]),          // 91: T17, D20
    Some(&[(20, 3), (16, 2)]),          // 92: T20, D16
    Some(&[(19, 3), (18, 2)]),          // 93: T19, D18
    Some(&[(18, 3), (20, 2)]),          // 94: T18, D20
    Some(&[(19, 3), (19, 2)]),          // 95: T19, D19
    Some(&[(20, 3), (18, 2)]),          // 96: T20, D18
    Some(&[(19, 3), (20, 2)]),          // 97: T19, D20
    Some(&[(20, 3), (19, 2)]),          // 98: T20, D19
    Some(&[(19, 3), (10, 1), (16, 2)]), // 99: T19, S10, D16
    Some(&[(20, 3), (20, 2)]),          // 100: T20, D20
    Some(&[(20, 3), (1, 1), (20, 2)]),  // 101: T20, 1, D20
    Some(&[(20, 3), (10, 1), (16, 2)]), // 102: T20, 10, D16
    Some(&[(20, 3), (3, 1), (20, 2)]),  // 103: T20, 3, D20
    Some(&[(18, 3), (18, 1), (16, 2)]), // 104: T18, 18, D16
    Some(&[(19, 3), (16, 1), (16, 2)]), // 105: T19, 16, D16
    Some(&[(20, 3), (14, 1), (16, 2)]), // 106: T20, 14, D16
    Some(&[(19, 3), (18, 1), (16, 2)]), // 107: T19, 18, D16
    Some(&[(20, 3), (16, 1), (16, 2)]), // 108: T20, 16, D16
    Some(&[(19, 3), (20, 1), (16, 2)]), // 109: T19, 20, D16
    Some(&[(20, 3), (18, 1), (16, 2)]), // 110: T20, 18, D16
    Some(&[(20, 3), (19, 1), (16, 2)]), // 111: T20, 19, D16
    Some(&[(20, 3), (12, 1), (20, 2)]), // 112: T20, 12, D20
    Some(&[(20, 3), (13, 1), (20, 2)]), // 113: T20, 13, D20
    Some(&[(20, 3), (14, 1), (20, 2)]), // 114: T20, 14, D20
    Some(&[(20, 3), (15, 1), (20, 2)]), // 115: T20, 15, D20
    Some(&[(20, 3), (16, 1), (20, 2)]), // 116: T20, 16, D20
    Some(&[(20, 3), (17, 1), (20, 2)]), // 117: T20, 17, D20
    Some(&[(20, 3), (18, 1), (20, 2)]), // 118: T20, 18, D20
    Some(&[(19, 3), (10, 3), (16, 2)]), // 119: T19, T10, D16
    Some(&[(20, 3), (20, 1), (20, 2)]), // 120: T20, 20, D20
    Some(&[(17, 3), (10, 3), (20, 2)]), // 121: T17, T10, D20
    Some(&[(18, 3), (20, 3), (4, 2)]),  // 122: T18, T20, D4
    Some(&[(19, 3), (16, 3), (9, 2)]),  // 123: T19, T16, D9
    Some(&[(20, 3), (16, 3), (8, 2)]),  // 124: T20, T16, D8
    Some(&[(25, 1), (20, 3), (20, 2)]), // 125: 25, T20, D20
    Some(&[(19, 3), (19, 3), (6, 2)]),  // 126: T19, T19, D6
    Some(&[(20, 3), (17, 3), (8, 2)]),  // 127: T20, T17, D8
    Some(&[(18, 3), (14, 3), (16, 2)]), // 128: T18, T14, D16
    Some(&[(19, 3), (16, 3), (12, 2)]), // 129: T19, T16, D12
    Some(&[(20, 3), (20, 3), (5, 2)]),  // 130: T20, T20, D5
    Some(&[(20, 3), (13, 3), (16, 2)]), // 131: T20, T13, D16
    Some(&[(20, 3), (16, 3), (12, 2)]), // 132: T20, T16, D12
    Some(&[(20, 3), (19, 3), (8, 2)]),  // 133: T20, T19, D8
    Some(&[(20, 3), (14, 3), (16, 2)]), // 134: T20, T14, D16
    Some(&[(20, 3), (17, 3), (12, 2)]), // 135: T20, T17, D12
    Some(&[(20, 3), (20, 3), (8, 2)]),  // 136: T20, T20, D8
    Some(&[(19, 3), (16, 3), (16, 2)]), // 137: T19, T16, D16
    Some(&[(20, 3), (18, 3), (12, 2)]), // 138: T20, T18, D12
    Some(&[(19, 3), (14, 3), (20, 2)]), // 139: T19, T14, D20
    Some(&[(20, 3), (16, 3), (16, 2)]), // 140: T20, T16, D16
    Some(&[(20, 3), (17, 3), (16, 2)]), // 141: T20, T17, D16
    Some(&[(20, 3), (14, 3), (20, 2)]), // 142: T20, T14, D20
    Some(&[(20, 3), (17, 3), (16, 2)]), // 143: T20, T17, D16
    Some(&[(20, 3), (20, 3), (12, 2)]), // 144: T20, T20, D12
    Some(&[(20, 3), (15, 3), (20, 2)]), // 145: T20, T15, D20
    Some(&[(20, 3), (18, 3), (16, 2)]), // 146: T20, T18, D16
    Some(&[(20, 3), (17, 3), (18, 2)]), // 147: T20, T17, D18
    Some(&[(20, 3), (16, 3), (20, 2)]), // 148: T20, T16, D20
    Some(&[(20, 3), (19, 3), (16, 2)]), // 149: T20, T19, D16
    Some(&[(20, 3), (18, 3), (18, 2)]), // 150: T20, T18, D18
    Some(&[(20, 3), (17, 3), (20, 2)]), // 151: T20, T17, D20
    Some(&[(20, 3), (20, 3), (16, 2)]), // 152: T20, T20, D16
    Some(&[(20, 3), (19, 3), (18, 2)]), // 153: T20, T19, D18
    Some(&[(20, 3), (18, 3), (20, 2)]), // 154: T20, T18, D20
    Some(&[(20, 3), (19, 3), (19, 2)]), // 155: T20, T19, D19
    Some(&[(20, 3), (20, 3), (18, 2)]), // 156: T20, T20, D18
    Some(&[(20, 3), (19, 3), (20, 2)]), // 157: T20, T19, D20
    Some(&[(20, 3), (20, 3), (19, 2)]), // 158: T20, T20, D19
    None, // 159: Kein klassischer Three-Dart-Checkout
    Some(&[(20, 3), (20, 3), (20, 2)]), // 160: T20, T20, D20
    Some(&[(20, 3), (17, 3), (25, 2)]), // 161: T20, T17, Bull
    None, // 162: Kein klassischer Three-Dart-Checkout
    None, // 163: Kein klassischer Three-Dart-Checkout
    Some(&[(20, 3), (18, 3), (25, 2)]), // 164: T20, T18, Bull
    None, // 165: Kein klassischer Three-Dart-Checkout
    None, // 166: Kein klassischer Three-Dart-Checkout
    Some(&[(20, 3), (19, 3), (25, 2)]), // 167: T20, T19, Bull
    None, // 168: Kein klassischer Three-Dart-Checkout
    None, // 169: Kein klassischer Three-Dart-Checkout
    Some(&[(20, 3), (20, 3), (25, 2)]), // 170: T20, T20, Bull
];

fn tuple_to_throw(tuple: (i32, i32)) -> Throw {
    return Throw { field: tuple.0, multiplier: tuple.1 };
}

fn convert_tuples_to_throw_array(tuples: &[(i32, i32)]) -> [Option<Throw>; 3] {
    let mut result = [None, None, None];
    for (i, &t) in tuples.iter().enumerate() {
        if i < 3 {
            result[i] = Some(tuple_to_throw(t));
        }
    }
    result
}

fn add_t20_throw(throws: &mut [Option<Throw>; 3], throw_index: usize) -> usize {
    if throw_index < 3 {
        throws[throw_index] = Some(Throw { field: 20, multiplier: 3 });
        throw_index + 1
    } else {
        throw_index
    }
}

fn add_checkout_throws(throws: &mut [Option<Throw>; 3], throw_index: usize, score: i32) -> usize {
    let mut current_index = add_t20_throw(throws, throw_index);

    if let Some(checkout_tuples) = CHECKOUT_TABLE_DOUBLE_OUT[score as usize] {
        for &tuple in checkout_tuples.iter() {
            if current_index >= 3 {
               return current_index;
            }
            throws[current_index] = Some(tuple_to_throw(tuple));
            current_index += 1;
        }
    }

    current_index
}

fn recommend_throws_no_checkout(mut score: i32, amount_of_throws: Option<i32>) -> [Option<Throw>; 3] {
    let mut amount_of_throws = amount_of_throws.unwrap_or(3);
    amount_of_throws = amount_of_throws.clamp(1, 3);

    let mut throws: [Option<Throw>; 3] = [None, None, None];
    let mut throw_index = 0;

    for _i in 1..=amount_of_throws {
        score = score - 60;

        if score <= 0 {
            return throws;
        }

        if score > 170 {
            throw_index = add_t20_throw(&mut throws, throw_index);
            continue;
        }

        if CHECKOUT_TABLE_DOUBLE_OUT[score as usize].is_some() {
            add_checkout_throws(&mut throws, throw_index, score);
            return throws;
        }

    }
    return throws;
}
fn fill_throws_with_recommendations(visit: &Visit, recommended_throws: &[Option<Throw>; 3]) -> [Option<Throw>; 3] {
    let mut filled_throws: [Option<Throw>; 3] = [None, None, None];
    let mut recommendation_index = 0;
    let mut filled_index = 0;

    for index in 0..3 {
        if filled_index >= 3 {
            break;
        }

        if let Some(throw) = visit.throws[index] {
            filled_throws[filled_index] = Some(throw);
            filled_index += 1;
        } else if let Some(throw) = recommended_throws[recommendation_index] {
            filled_throws[filled_index] = Some(throw);
            filled_index += 1;
            recommendation_index += 1;
        } else {
            recommendation_index += 1;
        }
    }

    filled_throws
}

pub fn recommend_throws(score: i32, visit : Option<Visit>) -> [Option<Throw>; 3] {
    log_debug!("Recommending throws for score: {}", score);

    let mut recommended_throws = [None, None, None];
    let mut amount_of_throws = 3;
    if let Some(ref visit) = visit {
        amount_of_throws = 3 - visit.get_darts_thrown();
    }

    if score > 170 {
        recommended_throws = recommend_throws_no_checkout(score, Some(amount_of_throws));
    } else if score >= 0{
        recommended_throws = match CHECKOUT_TABLE_DOUBLE_OUT[score as usize] {
            Some(throws) => convert_tuples_to_throw_array(throws),
            None => recommend_throws_no_checkout(score, Some(amount_of_throws)),
        };
    }

    if let Some(visit) = visit {
        recommended_throws = fill_throws_with_recommendations(&visit, &recommended_throws)
    }

    log_debug!("Recommended throws for score {}: {:?}", score, recommended_throws);
    recommended_throws
}
