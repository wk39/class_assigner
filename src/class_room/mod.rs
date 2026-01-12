// use std::ops::Not;

use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, Normal};

pub(crate) struct BuilderData {
    pub(crate) n_class: usize,
    pub(crate) students: Vec<Student>,
    pub(crate) dislike_group: Vec<Vec<StudentId>>,
    pub(crate) like_group: Vec<Vec<StudentId>>,

    //
    pub(crate) assign_result: Option<AssignResult>,
}

impl BuilderData {
    pub(crate) fn empty_class_setup(&self) -> AssignResult {
        AssignResult::new(self.n_class)
    }

    pub(crate) fn new_demo() -> Self {
        let n_class = 12;
        let n_students = 32 * n_class - 3;

        // let mut rng = rand::rng();
        let mut rng = StdRng::seed_from_u64(0);
        // let mut rng = StdRng::from_os_rng();
        let normal = Normal::new(60.0_f32, 15.0).unwrap();

        let students = (0..n_students)
            .map(|iid| {
                let gender = if rng.random_bool(0.7) {
                    Gender::Male
                } else {
                    Gender::Female
                };

                let score = normal.sample(&mut rng).clamp(0.0, 100.0);

                Student::new(iid, None, gender, score)
                //
            })
            .collect();

        let dislike_group = vec![
            //
            vec![10, 11, 12],
            vec![20, 21],
            vec![31, 32, 33],
            vec![40, 41],
        ];

        let like_group = vec![
            //
            vec![50, 51],
            vec![60, 61],
        ];

        Self {
            n_class,
            students,
            dislike_group,
            like_group,

            assign_result: None,
        }
    }
}

pub(crate) type StudentId = usize;

#[derive(Debug)]
pub(crate) struct Student {
    pub(crate) id: StudentId,
    pub(crate) alias: Option<String>,
    pub(crate) gender: Gender,
    pub(crate) score: f32,
}

impl Student {
    pub(crate) fn new(
        id: impl Into<StudentId>,
        alias: Option<String>,
        gender: Gender,
        score: f32,
    ) -> Self {
        Self {
            id: id.into(),
            alias,
            gender,
            score,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Gender {
    Male,
    Female,
}
impl Gender {
    // 출력 편의를 위한 메서드 (M/F)
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Gender::Male => "M",
            Gender::Female => "F",
        }
    }
}
#[derive(Debug, Clone)]
pub(crate) struct ClassRoom {
    pub(crate) number: usize,
    // name: Option<String>,
    pub(crate) students_male: Vec<StudentId>,
    pub(crate) students_female: Vec<StudentId>,

    pub(crate) score_average: Option<f32>,
    pub(crate) score_variance: Option<f32>,
    pub(crate) cost: Option<f32>,
}

impl ClassRoom {
    pub(crate) fn new(number: usize) -> Self {
        Self {
            number,
            students_male: Vec::new(),
            students_female: Vec::new(),
            score_average: None,
            score_variance: None,
            cost: None,
        }
    }
    fn is_empty(&self) -> bool {
        self.students_male.is_empty() && self.students_female.is_empty()
    }
    fn number_of_students(&self) -> usize {
        self.students_male.len() + self.students_female.len()
    }
    fn cal_score_average(&self, students: &[Student]) -> Option<f32> {
        if self.is_empty() {
            return None;
        }

        let male_average = {
            let score_sum = self
                .students_male
                .iter()
                .map(|iid| students.get(*iid).unwrap().score)
                .fold(0.0, |sum, score| sum + score);
            score_sum / self.students_male.len() as f32
        };
        let female_average = {
            let score_sum = self
                .students_female
                .iter()
                .map(|iid| students.get(*iid).unwrap().score)
                .fold(0.0, |sum, score| sum + score);
            score_sum / self.students_female.len() as f32
        };
        Some(
            (male_average * self.students_male.len() as f32
                + female_average * self.students_female.len() as f32)
                / ((self.students_male.len() + self.students_female.len()) as f32),
        )
    }
    fn cal_score_variance(&self, students: &[Student], average_score: f32) -> Option<f32> {
        if self.is_empty() {
            return None;
        }
        let sum = self
            .students_male
            .iter()
            .chain(self.students_female.iter())
            .map(|iid| (students.get(*iid).unwrap().score - average_score).powi(2))
            .fold(0.0, |sum, score| sum + score);
        Some(sum / (self.number_of_students() as f32))
    }
    pub(crate) fn update(&mut self, students: &[Student], grade_average_score: f32) {
        self.score_average = self.cal_score_average(students);

        self.score_variance = self
            .score_average
            .and_then(|mean| self.cal_score_variance(students, mean));

        self.cost = self
            .score_average
            .zip(self.score_variance)
            .map(|(mean, var)| (mean - grade_average_score).powi(2) + var);
    }

    pub(crate) fn print_layout(&self, students: &[Student]) {
        let empty = String::from("");
        self.students_male
            .iter()
            .chain(self.students_female.iter())
            .enumerate()
            .for_each(|(i, sid)| {
                let col = i % 5;
                let st = &students[*sid];
                let alias = st.alias.as_ref().unwrap_or(&empty);
                if col == 0 {
                    print!(
                        "  {:3}({:5.1} {}){} ",
                        st.id,
                        st.score,
                        st.gender.as_str(),
                        alias
                    );
                } else {
                    print!(
                        "| {:3}({:5.1} {}){} ",
                        st.id,
                        st.score,
                        st.gender.as_str(),
                        alias
                    );
                }
                if col == 4 {
                    println!();
                }
            });
        println!();
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AssignResult {
    pub(crate) rooms: Vec<ClassRoom>,
    // pub(crate) students: Vec<Student>,
    // pub(crate) dislike: Vec<Vec<StudentId>>,
    pub(crate) overall_cost: Option<f32>,
}

impl AssignResult {
    pub(crate) fn new(n_class: usize) -> Self {
        let rooms: Vec<_> = (0..n_class).map(ClassRoom::new).collect();
        Self {
            rooms,
            // dislike,
            overall_cost: None,
        }
    }
    pub(crate) fn number_of_classes(&self) -> usize {
        self.rooms.len()
    }

    pub(crate) fn cal_overall_cost(&mut self, students: &[Student], average_score: f32) {
        self.rooms
            .iter_mut()
            .for_each(|c| c.update(students, average_score));

        let var_max = self
            .rooms
            .iter()
            .map(|c| c.score_variance.unwrap())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let score_variance = var_max * self.number_of_classes() as f32;

        let score_average = self
            .rooms
            .iter()
            .fold(0.0, |prev, c| prev + c.cost.unwrap());

        self.overall_cost = Some(score_average + score_variance);
    }
}

fn assign_class(
    class_rooms: AssignResult,
    students: &[Student],
    dislike: &[Vec<StudentId>],
) -> AssignResult {
    let mut rng = StdRng::from_os_rng();

    // data
    let average_score =
        students.iter().fold(0.0, |prev, st| prev + st.score) / students.len() as f32;

    let mut students_male: Vec<_> = students
        .iter()
        .filter(|st| st.gender == Gender::Male)
        .collect();
    let mut students_female: Vec<_> = students
        .iter()
        .filter(|st| st.gender == Gender::Female)
        .collect();

    // assign
    //
    //
    students_male.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    students_female.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let mut rooms_final = class_rooms.clone();
    let mut idx = 0; // big initial value
    //
    let mut class_ids = (0..class_rooms.number_of_classes()).collect::<Vec<_>>();
    for i in 0..1_000 {
        // init.
        let mut rooms_cand = class_rooms.clone();

        students_male
            .chunks(rooms_cand.number_of_classes())
            .for_each(|stus| {
                class_ids.shuffle(&mut rng);
                class_ids
                    .iter()
                    .zip(stus.iter())
                    .for_each(|(&class_id, st)| {
                        rooms_cand.rooms[class_id].students_male.push(st.id);
                    });
            });

        students_female
            .chunks(rooms_cand.number_of_classes())
            .for_each(|stus| {
                class_ids.shuffle(&mut rng);
                class_ids
                    .iter()
                    .zip(stus.iter())
                    .for_each(|(&class_id, st)| {
                        rooms_cand.rooms[class_id].students_female.push(st.id);
                    });
            });

        //
        //
        rooms_cand.cal_overall_cost(students, average_score);
        //
        //

        if rooms_final.overall_cost.is_none()
            || rooms_cand.overall_cost.unwrap() < rooms_final.overall_cost.unwrap()
        {
            rooms_final = rooms_cand;
            idx = i;
        }
    }
    println!("{idx}");

    rooms_final
}

fn test() {
    let mut builder_data = BuilderData::new_demo();

    let class_setup = builder_data.empty_class_setup();

    //
    let assign_result = assign_class(
        class_setup,
        &builder_data.students,
        &builder_data.dislike_group,
    );

    for class in assign_result.rooms.iter() {
        let score = class.score_average.unwrap();
        let stdev = class.score_variance.unwrap().sqrt();
        println!(
            "class {:02} : #students m({:2})+f({:2})= {:2} score {:.2} score_stdev {:.2}",
            class.number,
            class.students_male.len(),
            class.students_female.len(),
            class.number_of_students(),
            score,
            stdev,
        );

        if false {
            class.print_layout(&builder_data.students);
        }
    }

    //
    builder_data.assign_result = Some(assign_result);
}
