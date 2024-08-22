package com.example.jk_core_question_example

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.lifecycle.lifecycleScope
import com.example.jk_core_question_example.ui.theme.JkcorequestionexampleTheme
import cn.mucang.android.jk.core.question.AsyncRt
import cn.mucang.android.jk.core.question.QuestionDb
import cn.mucang.android.jk.core.question.testAsync
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {
    private lateinit var dbPath: String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        AsyncRt.initRuntime()
        enableEdgeToEdge()
        setContent {
            JkcorequestionexampleTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Column(modifier = Modifier.fillMaxSize()) {
                        Button(
                            modifier = Modifier.padding(innerPadding),
                            onClick = {
                                val db = QuestionDb(dbPath)
                                val qs = db.getQuestions()
                                qs.forEach {
                                    println("xxxxxxxxxxx questionId:" + it.question)
                                }
                            }
                        ) {
                            Text("点击查询")
                        }
                        Button(
                            modifier = Modifier.padding(innerPadding),
                            onClick = {
                                lifecycleScope.launch {
                                    val res = testAsync(2U)
                                    println("textAsync:$res")
                                }
                            }
                        ) {
                            Text("测试异步方法")
                        }
                    }
                }
            }
        }

        val dbHelper = DatabaseHelper(this)
        dbPath = dbHelper.copyDatabaseFromAssets()
        println("dbPath: $dbPath")
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    JkcorequestionexampleTheme {
        Greeting("Android")
    }
}