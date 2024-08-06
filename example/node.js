const path = require('path');
const { Question, QuestionDb } = require('./index.node');

console.log(111, Question, QuestionDb);
const db = new QuestionDb(path.join(__dirname, './car.db'));
console.log(db);
const [q] = db.getQuestions();

db.close();
const q2 = db.getQuestionById(301976);

console.log(Question.prototype, q.question, q.id, q2);