/*2052526 信15 白俊豪*/
#include <iostream>
#include <iomanip>
using namespace std;
int main()
{
    int year, month, start_day = 1;
    int row;
    int spaces;
    int day = 1;
    //输入部分：
    while (1)
    {
        cout << "请输入年份(2000-2030)和月份(1-12) : ";
        cin >> year >> month;
        while (cin.fail() || !(year <= 2030 && year >= 2000 && month >= 1 && month <= 12))
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "输入错误,请重新输入" << endl;
            cout << "请输入年份(2000-2030)和月份(1-12) : ";
            cin >> year >> month;
        }
        // 
        cout << "请输入" << year << "年" << month << "月1日的星期(0-6表示星期日-星期六) : ";
        cin >> start_day;

        while (cin.fail() || start_day > 6 || start_day < 0)
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "输入错误,请重新输入" << endl;
            cout << "请输入" << year << "年" << month << "月1日的星期(0-6表示星期日-星期六) : ";
            cin >> start_day;
        }
        if (!(start_day <= 6 && start_day >= 0))
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "输入错误,请重新输入" << endl;
        }
        else
            break;
    }

    cout << endl;
    cout << year << "年" << month << "月的月历为 : " << endl;
    cout << "星期日  星期一  星期二  星期三  星期四  星期五  星期六" << endl;
    //判断闰年，初始化行号，日期
    row = start_day;
    day = 1;

    if (row != 0)
    {
        for (spaces = row; spaces > 1; spaces--)
        {
            cout << "        ";
        }
        cout << "    ";
    }
    if ((year % 100 != 0 && year % 4 == 0) || year % 400 == 0) //闰年
    {
        switch (month)
        {
            case 1:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);

                break;
            case 2:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 29);
                break;
            case 3:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 4:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 30);
                break;
            case 5:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 6:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 30);
                break;
            case 7:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 8:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 9:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 30);
                break;
            case 10:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 11:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 30);
                break;
            case 12:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
        }
    }
    else //平年
    {
        switch (month)
        {
            case 1:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);

                break;
            case 2:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 28);
                break;
            case 3:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 4:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 30);
                break;
            case 5:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 6:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 30);
                break;
            case 7:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 8:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 9:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 30);
                break;
            case 10:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
            case 11:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 30);
                break;
            case 12:
                do
                {
                    if (row == 0)
                    {
                        cout << setw(4) << day;
                        row++;
                    }
                    else if (row != 0)
                    {
                        cout << setw(8) << day;
                        row++;
                    }
                    if (row > 6) //换行重置row的值
                    {
                        row = 0;
                        cout << endl;
                    }
                    day++;
                } while (day <= 31);
                break;
        }
    }
    return 0;
}
