use std::collections::HashSet;

use bbox::context::Context;
use bbox::policy::AbstractPolicy;

use crate::apikey::ApiKey;
use crate::backend::MySqlBackend;
use crate::questions::LectureAnswer;

struct AnswerAccessPolicy {
    admins: HashSet<String>,
}

// Content of answer column can only be accessed by:
//   1. The user who submitted the answer (`user_id == me`);
//   2. The admin(s) (`is me in set<admins>`);
//   3. Any student who is leading discussion for the lecture
//      (`P(me)` alter. `is me in set<P(students)>`);

impl AbstractPolicy<LectureAnswer, ApiKey, &mut MySqlBackend> for AnswerAccessPolicy {
    fn check(&self, data: LectureAnswer, context: Context<ApiKey>, _db: &mut MySqlBackend) -> bool {
        let user = &context.get_user().unwrap().user;

        data.user.internal_unbox() == user.internal_unbox()
            || self.admins.contains(user.internal_unbox())
            // || db.prep_exec("SELECT ID FROM students WHERE <P>", vec![]).contains(user)
    }
}