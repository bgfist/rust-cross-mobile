//
//  ContentView.swift
//  jk-core-question-example
//
//  Created by 周旺 on 2024/8/12.
//

import SwiftUI
import JkCoreQuestion

class Params {
    var questionDb: QuestionDb? = nil
    var checkTimes: Int = 0
    var checkDuration: TimeInterval = 0
}

struct ContentView: View {
    var params: Params
    
    var body: some View {
        VStack {
            Button("Query") {
                self.params.checkTimes = 0
                self.params.checkDuration = 0
                self.checkQuestions()
            }
            Button("Save") {
                
            }
            Button("Delete") {
                
            }
            Button("Request") {
                
            }
        }
        .padding()
    
    }
    
    init(questionDb: QuestionDb? = nil, checkTimes: Int = 0, checkDuration: TimeInterval = 0) {
        let params = Params.init()
        params.checkTimes = checkTimes
        params.checkDuration = checkDuration
        
        if (questionDb == nil) {
            let path = Bundle.main.path(forResource: "car", ofType: "db") ?? ""
            params.questionDb = QuestionDb.init(dbPath: path)
        } else {
            params.questionDb = questionDb
        }
        
        self.params = params
    }
    
    
    func checkQuestions() {
        let maxTimes = 10
        let currentCheckTimes = self.params.checkTimes + 1
        
        if currentCheckTimes > maxTimes {
            //计算平均耗时
            let averageDuration = self.params.checkDuration / Double(self.params.checkTimes)
            print("平均耗时：\(averageDuration)")
            return
        }
        
        var costTime: TimeInterval = 0
            
        if let questionDb = self.params.questionDb {
            let totalCount = 10000
            let startTime = Date.now.timeIntervalSince1970
            print("第\(currentCheckTimes)轮，查询数据库 \(totalCount) 次，开始>>>>")
            for i in 1...totalCount {
                let result = try? questionDb.getQuestions()
                if let result = result {
                    let count = result.count
                }
            }
            print("第\(currentCheckTimes)轮，查询数据库 \(totalCount) 次，结束<<<<")
            let endTime = Date.now.timeIntervalSince1970
            costTime = endTime - startTime
            print("第\(currentCheckTimes)轮，总共耗时：\(costTime)")
        }
            
        self.params.checkTimes = currentCheckTimes
        self.params.checkDuration += costTime
        
        DispatchQueue.main.asyncAfter(deadline: DispatchTime.now() + DispatchTimeInterval.seconds(1)) {
            self.checkQuestions()
        }
       
    }
}

#Preview {
    ContentView()
}


