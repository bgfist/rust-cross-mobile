package com.example.jk_core_question_example

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.example.jk_core_question_example.ui.theme.JkcorequestionexampleTheme
import cn.mucang.android.jk.core.question.QuestionDb

class MainActivity : ComponentActivity() {
    private lateinit var dbPath: String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            JkcorequestionexampleTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Greeting(
                        name = "Android",
                        modifier = Modifier.padding(innerPadding)
                    )
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