using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Net;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace KatFetch
{
    public partial class Form1 : Form
    {
        private int stage = 0;
        private int index = 0;
        private int index2 = 0;
        private List<Solution> solutions = new List<Solution>();
        private List<Solution> solutions2;
        public Form1()
        {
            InitializeComponent();
        }

        private void Form1_Load(object sender, EventArgs e)
        {
            wb.Navigate("https://open.kattis.com/login/");
        }

        private void HandleMup(object sender, WebBrowserDocumentCompletedEventArgs e)
        {
            if(stage == 0)
            {
                wb.Document.GetElementById("user_input").SetAttribute("value", textBox1.Text);
                wb.Document.GetElementById("password_input").SetAttribute("value", textBox2.Text);
                wb.Document.GetElementsByTagName("input")[4].InvokeMember("click");
                stage++;
            }
            else if(stage == 1)
            {
                var p = wb.Document.GetElementsByTagName("a");
                wb.Document.GetElementsByTagName("a")[8].InvokeMember("click");
                stage++;
            }
            else if(stage == 2)
            {
                try
                {
                    var ar = wb.Document.GetElementsByTagName("tbody")[1].GetElementsByTagName("tr");

                    foreach (HtmlElement i in ar)
                    {
                        if (i.Children[3].OuterText != "Accepted")
                            continue;
                        var s = new Solution();
                        s.Url = i.Children[0].Children[0].GetAttribute("href");
                        s.ID = i.Children[2].Children[0].GetAttribute("href").Split('/')[4];
                        s.Language = i.Children[5].InnerText;
                        if (!solutions.Contains(s))
                        {
                            Console.WriteLine(s);
                            solutions.Add(s);
                        }
                    }
                    var btn = wb.Document.GetElementById("problem_list_next");
                    //if (btn == null || btn.OuterHtml.Contains("disabled"))
                    {
                        progressBar1.Maximum = solutions.Count;
                        progressBar1.Value = 0;
                        solutions2 = solutions.Take(solutions.Count / 2).ToList();
                        solutions = solutions.Skip(solutions.Count / 2).ToList();
                        stage++;
                        wb.Navigate(solutions[index].Url);
                        wb2.Navigate(solutions2[index2].Url);
                    }
                    //else
                      //  btn.InvokeMember("click");
                }
                catch (Exception ass)
                {
                    Console.WriteLine(ass.StackTrace);
                }
            }
            else if(stage == 3)
            {
                if (!Directory.Exists("./dl"))
                    Directory.CreateDirectory("./dl");

                if (index < solutions.Count)
                {
                    HtmlElement p = null;
                    HtmlElement ap = null;
                    foreach (HtmlElement ep in wb.Document.GetElementsByTagName("a"))
                    {
                        if (ep.InnerText == "download")
                        {
                            p = ep;
                            break;
                        }
                    }
                    foreach (HtmlElement ep in wb.Document.GetElementsByTagName("div"))
                    {
                        if (ep.GetAttribute("className") == "submission_code_wrapper")
                        {
                            ap = ep;
                            break;
                        }
                    }
                    var url = p?.GetAttribute("href");

                    var name = url.Split('/')[6].Split('.');

                    File.WriteAllText("./dl/" + solutions[index].ID + "." + name[1], ap.InnerText);

                    index++;
                    progressBar1.Invoke(new Action(() => progressBar1.Increment(1)));
                    wb.Navigate(solutions[index].Url);
                }
                else
                {
                    stage++;
                    wb.Navigate("https://open.kattis.com/login/email");
                }
            }
        }

        class Solution : IEquatable<Solution>
        {
            public string Language { get; set; }
            public string Url { get; set; }
            public string ID { get; set; }

            public bool Equals(Solution other)
            {
                return Language ==  other.Language && ID == other.ID;
            }

            public override string ToString()
            {
                return $"ID: {ID} Lang: {Language}";
            }
            public override bool Equals(object obj)
            {
                return Equals((Solution)obj);
            }
            public override int GetHashCode()
            {
                return Language.GetHashCode() + ID.GetHashCode();
            }
        }

        private void button1_Click(object sender, EventArgs e)
        {
            wb.DocumentCompleted += HandleMup;
            wb.Navigate("https://open.kattis.com/login/email");
            wb2.DocumentCompleted += Wb2_DocumentCompleted;
            wb2.ScriptErrorsSuppressed = true;
            wb.ScriptErrorsSuppressed = true;
        }

        private void Wb2_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
        {
            if (!Directory.Exists("./dl"))
                Directory.CreateDirectory("./dl");

            if (index2 < solutions2.Count)
            {
                HtmlElement p = null;
                HtmlElement ap = null;
                foreach (HtmlElement ep in wb2.Document.GetElementsByTagName("a"))
                {
                    if (ep.InnerText == "download")
                    {
                        p = ep;
                        break;
                    }
                }
                foreach (HtmlElement ep in wb2.Document.GetElementsByTagName("div"))
                {
                    if (ep.GetAttribute("className") == "submission_code_wrapper")
                    {
                        ap = ep;
                        break;
                    }
                }
                var url = p?.GetAttribute("href");
              
                var name = url.Split('/')[6].Split('.');

                File.WriteAllText("./dl/" + solutions2[index2].ID + "." + name[1], ap.InnerText);
                index2++;
                progressBar1.Invoke(new Action(() => progressBar1.Increment(1)));
                wb2.Navigate(solutions2[index2].Url);
            }
            else
            {
                wb.Navigate("https://open.kattis.com/login/email");
            }
        }

        private void button2_Click(object sender, EventArgs e)
        {
            wb.DocumentCompleted += HandleMup;
            stage = 1;
            wb.Navigate("https://open.kattis.com/");
            wb2.DocumentCompleted += Wb2_DocumentCompleted;
            wb2.ScriptErrorsSuppressed = true;
            wb.ScriptErrorsSuppressed = true;
        }

        private void wb_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
        {

        }

        private void button3_Click(object sender, EventArgs e)
        {
            var p = Directory.GetFiles("./dl");
            
            foreach (var f in p)
            {
                var arr = File.ReadAllLines(f);
                var list = new List<string>();
                
                var index = 0;
                while (IsWhite(arr[index]))
                    index++;
                list.Add(arr[index]);
                bool add = true;
                for (int i = index+1; i < arr.Length; i++)
                {
                    add = !add;
                    if (add)
                        list.Add(arr[i]);
                }
                File.WriteAllLines(f, list.ToArray()); 
            }
        }
        private bool IsWhite(string s)
        {
            bool white = true;
            foreach (var c in s)
            {
                if(c != ' ' && c != '\t')
                {
                    white = false;
                    break;
                }
            }
            return white;
        }
    }
}
