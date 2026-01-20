/*2052521 信13 张耀尹*/
#include<iostream>
#include<iomanip>
#include<Windows.h>
#include <conio.h>
#include "mine_sweeper.h"
#include "cmd_console_tools.h"
using namespace std;
void menu()
{
    cout << "---------------------------------" << endl;
    cout << "1.选择难度并显示内部数组" << endl;
    cout << "2.输入初始位置并显示被打开的初始区域" << endl;
    cout << "3.内部数组基础版" << endl;
    cout << "4.内部数组完整版（标记、运行时间）" << endl;
    cout << "5.画出伪图形化框架并显示内部数据" << endl;
    cout << "6.检测鼠标位置和合法性（左键单击退出）" << endl;
    cout << "7.鼠标选择初始位置并显示被打开的初始区域" << endl;
    cout << "8.伪图形界面基础版" << endl;
    cout << "9.伪图形界面完整版" << endl;
    cout << "0.退出游戏" << endl;
    cout << "---------------------------------" << endl;
    cout << endl;
    cout << "[请选择:]";
}

void choose(char ch)
{
    if (ch == '1')
        fun1(ch);
    else if (ch == '2')
        fun2(ch);
    else if (ch == '3')
        fun3(ch);
    else if (ch == '4')
        fun4(ch);
    else if (ch == '5')
        fun5(ch);
    else if (ch == '6')
        fun6(ch);
    else if (ch == '7')
        fun7(ch);
    else if (ch == '8')
        fun8(ch);
    else if (ch == '9')
        fun9(ch);
    backmenu(ch);
}

void backmenu(char choice)
{
    while (getchar() != '\n')
        ;
    cct_cls();
    menu();
}

void sub_menu(char& ch)
{
    cct_cls();
    cout << "请选择难度：" << endl;
    cout << "1.初级(9 * 9 - 10颗雷)" << endl;
    cout << "2.中级(16 * 16 - 40颗雷)" << endl;
    cout << "3.高级(16 * 30 - 99颗雷)" << endl;
    cout << "请输入[1..3]：";
    while (1)
    {
        ch = _getch();
        if (ch == '1' || ch == '2' || ch == '3')
            break;
    }
    cct_cls();
    cout << "内部数组：" << endl;
}
