import fs from '@ohos.file.fs';
import hilog from '@ohos.hilog';
import common from '@ohos.app.ability.common';

export class TiKuUtils {
  static readonly domain = 0x111
  static readonly tag = "TiKuUtils"
  /**
   * 打包内置的题库文件
   */
  static readonly RAW_FILE_NAME = "car.db"
  /**
   * 数据库目录
   */
  static readonly DB_DIR = "/rdb"
  /**
   * 数据库目录的题库文件
   */
  static readonly DB_NAME = "question.db"

  /**
   * 将内置db拷贝到数据库目录
   */
  static async copyRawfileToDatabase(context: common.ApplicationContext) {
    try {
      let dbDir = `${context.databaseDir}${TiKuUtils.DB_DIR}`
      let dirExist = await fs.access(dbDir)
      if (!dirExist) {
        await fs.mkdir(dbDir)
        hilog.info(TiKuUtils.domain, TiKuUtils.tag, `copyDb2Database mkdir=${dbDir}...`)
      }
      let destPath = `${dbDir}/${TiKuUtils.DB_NAME}`
      let destFileExist = await fs.access(destPath)
      if (destFileExist) {
        hilog.info(TiKuUtils.domain, TiKuUtils.tag, "question db  exist...")
        return
      }
      hilog.info(TiKuUtils.domain, TiKuUtils.tag, "copyDb2Database start...")
      let srcFd = await context.resourceManager.getRawFd(TiKuUtils.RAW_FILE_NAME)
      let destFile = await fs.open(destPath, fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE)
      let buffer = new ArrayBuffer(1024 * 1024)
      let readLength = 0
      let readTotalLength = 0
      let length = srcFd.length
      while ((readLength = await fs.read(srcFd.fd, buffer, {
        offset: srcFd.offset + readTotalLength,
        length: length > buffer.byteLength ? 1024 * 1024 : length
      })) > 0) {
        readTotalLength += readLength
        console.log('aaaaa' + readLength);
        await fs.write(destFile.fd, buffer, { length: readLength })
        length -= buffer.byteLength
      }
      await fs.close(srcFd.fd)
      hilog.info(TiKuUtils.domain, TiKuUtils.tag, `dest file length=${fs.statSync(destPath).size}...`)
      await fs.close(destFile)
      buffer = new ArrayBuffer(0)
      hilog.info(TiKuUtils.domain, TiKuUtils.tag, `copyDb2Database success...`)
    } catch (e) {
      hilog.info(TiKuUtils.domain, TiKuUtils.tag, `copyDb2Database error:${e}}`)
    }
  }
}