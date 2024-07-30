const { Question, QuestionDb } = require('./libjk_core_question.node');

const db = new QuestionDb('./car.db');
console.log(db);
const [q] = db.getQuestions();

console.log(Question.prototype, q.question);