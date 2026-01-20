/*2052526 信15 白俊豪*/
#include <iostream>
#include <cmath>
#include <iomanip>
#define pi 3.14159
using namespace std;
int main()
{
    int a, b, angle;
    double S;
    cout << "请输入三角形的两边及其夹角（角度） :  " << endl;
    cin >> a >> b >> angle;
    S = a * b * sin(angle * pi / 180) / 2;
    cout << setiosflags(ios::fixed) << setprecision(3) << "三角形面积为 ： " << S << endl;

    return 0;
}

//Done.