import { hilog } from '@kit.PerformanceAnalysisKit';
import { hello } from '@mc/jk-core-question';

@Entry
@Component
struct Index {
  @State message: string = 'Hello World';

  build() {
    Row() {
      Column() {
        Text(this.message)
          .fontSize(50)
          .fontWeight(FontWeight.Bold)
          .onClick(() => {
            // const db = new QuestionDb("");
            // const questions = db.getQuestions();
            // hilog.info(0x0000, 'jk_core_question', questions[0].question);
            hilog.info(0x0000, 'jk_core_question', hello());
          })
      }
      .width('100%')
    }
    .height('100%')
  }
}
