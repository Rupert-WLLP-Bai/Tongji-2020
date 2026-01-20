/*2052526 信15 白俊豪*/
#include <iostream>
#include <cmath>
#include <iomanip>
using namespace std;
int main()
{
    int i1, i2, i3, i4, i5, i6, i7, i8, i9, i10;
    int i_1;
    double i_2;
    int temp1, temp2, temp3, temp4, temp5, temp6, temp7, temp8, temp9;
    double temp_1;
    double num, num_int;
    double num_decimal_part;
    cout << "请输入[0-100亿)之间的数字,小数点后最多两位：" << endl;
    cin >> num;
    num_int = num - fmod(num, 1);
    cout << setiosflags(ios::fixed) << setprecision(2);

    /*测试代码
    cout << num << endl;
    cout << num_int <<endl;
    */

    //整数部分的计算：
    i1 = int(fmod(num_int, 10));
    temp1 = int(num_int / 10);
    i2 = temp1 % 10;
    temp2 = temp1 / 10;
    i3 = temp2 % 10;
    temp3 = temp2 / 10;
    i4 = temp3 % 10;
    temp4 = temp3 / 10;
    i5 = temp4 % 10;
    temp5 = temp4 / 10;
    i6 = temp5 % 10;
    temp6 = temp5 / 10;
    i7 = temp6 % 10;
    temp7 = temp6 / 10;
    i8 = temp7 % 10;
    temp8 = temp7 / 10;
    i9 = temp8 % 10;
    temp9 = temp8 / 10;
    i10 = temp9 % 10;

    //小数部分的计算：
    num_decimal_part = fmod(num, 1);
    i_1 = int((num_decimal_part * 10) + 1e-4); //如果只输入一位小数,用于排除误差
    temp_1 = num_decimal_part * 100 + 1e-3;    //同样的方法调整temp_1,排除尾数为零时可能出现的bug
    i_2 = fmod(temp_1, 10);
    //方案二：将尾数加上一个较小的数，再向下取整(用于排除最后一位赋值不准确的误差)
    i_2 = int(i_2 + 1e-4);

    //输出部分：
    cout << "十亿位  : " << i10 << endl;
    cout << "亿位    : " << i9 << endl;
    cout << "千万位  : " << i8 << endl;
    cout << "百万位  : " << i7 << endl;
    cout << "十万位  : " << i6 << endl;
    cout << "万位    : " << i5 << endl;
    cout << "千位    : " << i4 << endl;
    cout << "百位    : " << i3 << endl;
    cout << "十位    : " << i2 << endl;
    cout << "圆      : " << i1 << endl;
    cout << "角      : " << i_1 << endl;
    cout << "分      : " << int(i_2) << endl;

    return 0;
}

//Version 4 Done.