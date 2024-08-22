import { common } from '@kit.AbilityKit';
import { relationalStore } from '@kit.ArkData';
import { hash } from '@kit.CoreFileKit';
import { TiKuUtils } from './TiKuUtils';
import fs from '@ohos.file.fs';
import { QuestionDb, testAsync } from '@mc/jk_core_question';

@Entry
@Component
struct Index {
  @State copyResult: string = "拷贝结果："
  @State readBeforeResult: string = "读取前结果："
  @State readAfterResult: string = "读取后结果："
  @State dbPath: string = ""

  build() {
    Row() {
      Scroll() {
        Column() {
          Button("1、拷贝rawfile目录原始文件到数据库路径")
            .fontSize(18)
            .fontWeight(FontWeight.Bold)
            .margin({ top: 20 })
            .onClick(async () => {
              const dbPath = await this.copyRawfile()
              this.dbPath = dbPath
            })
          Text(this.copyResult)
            .fontSize(16)
            .fontWeight(FontWeight.Bold)

          Button("2、native读取数据库")
            .fontSize(18)
            .fontWeight(FontWeight.Bold)
            .margin({ top: 20 })
            .onClick(() => {
              this.readDb()
            })

          Button("3、rust读取数据库")
            .enabled(!!this.dbPath)
            .fontSize(18)
            .fontWeight(FontWeight.Bold)
            .margin({ top: 20 })
            .onClick(() => {
              this.rustReadDb()
            })
          
          Button("4、rust测试异步方法")
            .enabled(!!this.dbPath)
            .fontSize(18)
            .fontWeight(FontWeight.Bold)
            .margin({ top: 20 })
            .onClick(() => {
              testAsync(2).then(res => {
                this.copyResult = "异步方法返回: " + res;
              })
            })

          Text(this.readBeforeResult)
            .fontSize(16)
            .fontWeight(FontWeight.Bold)
          Text(this.readAfterResult)
            .fontSize(16)
            .fontWeight(FontWeight.Bold)
        }
        .width('100%')
        .height('100%')
        .justifyContent(FlexAlign.Start)
        .alignItems(HorizontalAlign.Start)
      }
    }
    .height('100%')
  }

  async copyRawfile(): Promise<string> {
    let context: common.ApplicationContext = getContext(this).getApplicationContext()
    let filePath = `${context.databaseDir}${TiKuUtils.DB_DIR}/${TiKuUtils.DB_NAME}`
    if (await fs.access(filePath)) {
      let md5 = await hash.hash(filePath, 'md5')
      this.copyResult = `拷贝结果：文件已存在,md5：${md5}`
      return filePath
    }
    await TiKuUtils.copyRawfileToDatabase(context)
    if (await fs.access(filePath)) {
      let md5 = await hash.hash(filePath, 'md5')
      this.copyResult = `拷贝结果：成功,md5：${md5}`
    } else {
      this.copyResult = "拷贝结果：失败～"
    }

    return filePath
  }

  async readDb(): Promise<void> {

    let context: common.ApplicationContext = getContext(this).getApplicationContext()

    let rdbStore: relationalStore.RdbStore
    let resultSet: relationalStore.ResultSet
    let majorVersion: number = 0
    let smallVersion: number = 0

    try {
      let config: relationalStore.StoreConfig = {
        name: TiKuUtils.DB_NAME,
        securityLevel: relationalStore.SecurityLevel.S1,
        isReadOnly: true
      }

      let filePath = `${context.databaseDir}${TiKuUtils.DB_DIR}/${TiKuUtils.DB_NAME}`
      let md5 = await hash.hash(filePath, 'md5')
      this.readBeforeResult = `majorVersion=${majorVersion},smallVersion=${smallVersion},md5：${md5}`

      rdbStore = await relationalStore.getRdbStore(context, config)

      resultSet = await rdbStore.querySql("select max(version) from t_version")
      if (resultSet.goToNextRow()) {
        smallVersion = resultSet.getLong(0)
      }
      resultSet.close()

      resultSet = await rdbStore.querySql("select major_version from t_version")
      if (resultSet.goToNextRow()) {
        majorVersion = resultSet.getLong(0)
      }
      resultSet.close()

      let md52 = await hash.hash(filePath, 'md5')
      this.readAfterResult = `majorVersion=${majorVersion},smallVersion=${smallVersion},md5：${md52}`

    } catch (ex) {

    }
  }

  async rustReadDb(): Promise<void> {
    const db = new QuestionDb(this.dbPath);
    const questions = db.getQuestions();
    this.readBeforeResult = `db.getQuestions: {count: ${questions.length}} {${questions[0].question}}`;

    const qId = questions[0].id;
    const question = db.getQuestionById(qId);
    this.readAfterResult = `db.getQuestionById(${qId}): ${question ? question.optionA : null}`;
  }
}