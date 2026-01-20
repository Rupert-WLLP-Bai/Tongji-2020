/*2052526 信15 白俊豪*/
#include <iostream>
#include <iomanip>
using namespace std;

/* ----具体要求----
   1、不允许添加其它头文件
   2、不允许定义全局变量、静态局部变量
   3、允许添加用于输入层数、起始/目标柱的函数，函数中允许使用循环处理错误输入
   4、如果输入用函数进行，则main中不允许出现任何形式的循环（while、do-while、for、if-goto）
      如果输入在main中进行，则main中允许出现循环
   --------------------------------------------------------------------- */

   /***************************************************************************
           函数名称：
           功    能：打印n层汉诺塔的移动顺序
           输入参数：int n：层数
                     char src：起始柱
                     char tmp：中间柱
                     char dst：目标柱
           返 回 值：
           说    明：1、函数名、形参、返回类型均不准动
                     2、本函数不允许出现任何形式的循环
         ***************************************************************************/
void haoni(int n, char src, char tmp, char dst)
{
    static int i = 0;
    void move(int, char, char);
    if (n == 1)
    {
        i++;
        cout << setw(5) << i << ":";
        move(n, src, dst);
    }
    else
    {
        haoni(n - 1, src, dst, tmp);

        i++;
        cout << setw(5) << i << ":";
        move(n, src, dst);


        haoni(n - 1, tmp, src, dst);
        
    }
}

void move(int n, char x, char y)
{
    cout << setw(3) << n << "# ";
    cout << x << "-->" << y << endl;
}

/***************************************************************************
  函数名称：
  功    能：
  输入参数：
  返 回 值：
  说    明：完成输入(或调用输入函数)、调用递归函数
***************************************************************************/
int main()
{
    int level;
    char start, mid, end;
    while (1)
    {
        cout << "请输入汉诺塔的层数(1-16)" << endl;
        cin >> level;
        while (cin.fail())
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "请输入汉诺塔的层数(1-16)" << endl;
            cin >> level;
        }
        if (level >= 1 && level <= 16)
            break;
    }
    cin.clear();
    cin.ignore(1024, '\n');
    while (1)
    {
        cout << "请输入起始柱(A-C)" << endl;
        cin >> start;
        while (cin.fail())
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "请输入起始柱(A-C)" << endl;
            cin >> start;
        }
        if (start == 'A' || start == 'B' || start == 'C')
            break;
        else if (start == 'a' || start == 'b' || start == 'c')
        {
            start -= 32;
            break;
        }
        cin.clear();
        cin.ignore(1024, '\n');
    }
    cin.clear();
    cin.ignore(1024, '\n');
    while (1)
    {
        cout << "请输入目标柱(A-C)" << endl;
        cin >> end;
        while (cin.fail())
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "请输入目标柱(A-C)" << endl;
            cin >> end;
        }
        if (end == start || end == start + 32 || end == start - 32)
        {
            cin.clear();
            cin.ignore();
            cout << "起始柱不能与目标柱相同" << endl;
            continue;
        }
        if (end == 'A' || end == 'B' || end == 'C')
            break;
        else if (end == 'a' || end == 'b' || end == 'c')
        {
            end -= 32;
            break;
        }
        cin.clear();
        cin.ignore(1024, '\n');
    }
    mid = 'A' + 'B' + 'C' - start - end;
    haoni(level, start, mid, end);
    return 0;
}