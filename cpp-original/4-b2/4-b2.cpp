/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;

int zeller(int y, int m, int d)
{
    int w, c, y1, start;
    if (m >= 3 && m <= 14)
    {
        y1 = y % 100; //得到公式中Y的值
        c = y / 100;  //得到公式中C的值
    }
    else
    {
        m += 12;
        y1 = (y - 1) % 100; //得到公式中Y的值
        c = (y - 1) / 100;  //得到公式中C的值
    }
    w = y1 + y1 / 4 + c / 4 - 2 * c + (13 * (m + 1) / 5) + d - 1;
    while (w <= 0)
        w += 7;
    start = w % 7;
    return start;
}

int main()
{
    int y, m, d, result;
    bool is_leap_year = 0;
    //输入以及判断合法性(1.平年,闰年 2.非法字符)
    while (1)
    {
        cout << "输入年月日,范围是(1900.1.1-2099.12.31) : " << endl;
        cin >> y >> m >> d;
        while (cin.fail())
        {
            cin.clear();
            cin.ignore(1024, '\n');
            cout << "输入错误,请重新输入" << endl;
            cout << "输入年月日,范围是(1900.1.1-2099.12.31) : " << endl;
            cin >> y >> m >> d;
        }

        if (y >= 1900 && y <= 2099)
        {
            if ((y % 4 == 0 && y % 100 != 0) || y % 400 == 0)
                is_leap_year = true;
            if (m >= 1 && m <= 12)
            {
                if (m == 1 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10 || m == 12)
                {
                    if (d < 1 || d > 31)
                    {
                        cout << "输入错误,请重新输入" << endl;
                    }
                    else
                        break;
                }
                else if (m == 4 || m == 6 || m == 9 || m == 11)
                {
                    if (d < 1 || d > 30)
                    {
                        cout << "输入错误,请重新输入" << endl;
                    }
                    else
                        break;
                }

                else
                {
                    if (is_leap_year)
                    {
                        if (d < 1 || d > 29)
                        {
                            cout << "输入错误,请重新输入" << endl;
                        }
                        else
                            break;
                    }
                    else
                    {
                        if (d < 1 || d > 28)
                        {
                            cout << "输入错误,请重新输入" << endl;
                        }
                        else
                            break;
                    }
                }
            }
            else
            {
                cout << "输入错误,请重新输入" << endl;
            }
        }
        else
        {
            cout << "输入错误,请重新输入" << endl;
        }
    }

    //结果处理
    result = zeller(y, m, d);
    switch (result)
    {
        case 0:
            cout << "星期日";
            break;
        case 1:
            cout << "星期一";
            break;
        case 2:
            cout << "星期二";
            break;
        case 3:
            cout << "星期三";
            break;
        case 4:
            cout << "星期四";
            break;
        case 5:
            cout << "星期五";
            break;
        case 6:
            cout << "星期六";
            break;
    }
    cout << endl;
    return 0;
}