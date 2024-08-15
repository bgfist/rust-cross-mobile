import { hilog } from '@kit.PerformanceAnalysisKit';
import { QuestionDb } from '@mc/jk_core_question';

@Entry
@Component
struct Index {
  @State message: string = 'Hello World';

  build() {
    Row() {
      Column() {
        Text(this.message)
          .width(100)
          .height(50)
          .fontColor(Color.Red)
          .fontSize(50)
          .fontWeight(FontWeight.Bold)
          .onClick(() => {
            const db = new QuestionDb("");
            const questions = db.getQuestions();
            hilog.info(0x0000, 'jk_core_question', questions[0].question);
          })
      }
      .width('100%')
    }
    .height('100%')
  }
}
