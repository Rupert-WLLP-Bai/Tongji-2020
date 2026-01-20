/*2052526 信15 白俊豪*/
#include <iostream>
#include <cmath>
using namespace std;
int main()
{
    int i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i_1, i_2, temp1, temp2, temp3, temp4, temp5, temp6, temp7, temp8, temp9;
    double temp_1, num, num_int, num_decimal_part;
    bool i10_19_exist = 1, zero_1_exist = 0, zero_2_exist = 0, zheng_exist = 0;
    cout << "请输入[0-100亿)之间的数字,小数点后最多两位：" << endl;
    cin >> num;
    num_int = num - fmod(num, 1);
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
    num_decimal_part = fmod(num, 1);
    i_1 = int((num_decimal_part * 10) + 1e-4);
    temp_1 = num_decimal_part * 100 + 1e-3;
    i_2 = int(fmod(temp_1, 10) + 1e-4);
    switch (i10) //十亿位
    {
        case 1:
            cout << "壹拾";
            break;
        case 2:
            cout << "贰拾";
            break;
        case 3:
            cout << "叁拾";
            break;
        case 4:
            cout << "肆拾";
            break;
        case 5:
            cout << "伍拾";
            break;
        case 6:
            cout << "陆拾";
            break;
        case 7:
            cout << "柒拾";
            break;
        case 8:
            cout << "捌拾";
            break;
        case 9:
            cout << "玖拾";
            break;
    }
    switch (i9) //亿位
    {
        case 1:
            cout << "壹";
            break;
        case 2:
            cout << "贰";
            break;
        case 3:
            cout << "叁";
            break;
        case 4:
            cout << "肆";
            break;
        case 5:
            cout << "伍";
            break;
        case 6:
            cout << "陆";
            break;
        case 7:
            cout << "柒";
            break;
        case 8:
            cout << "捌";
            break;
        case 9:
            cout << "玖";
            break;
    }
    if (i10 != 0 || i9 != 0)
        cout << "亿";
    else
        i10_19_exist = 0;
    switch (i8) //千万位
    {
        case 1:
            cout << "壹仟";
            break;
        case 2:
            cout << "贰仟";
            break;
        case 3:
            cout << "叁仟";
            break;
        case 4:
            cout << "肆仟";
            break;
        case 5:
            cout << "伍仟";
            break;
        case 6:
            cout << "陆仟";
            break;
        case 7:
            cout << "柒仟";
            break;
        case 8:
            cout << "捌仟";
            break;
        case 9:
            cout << "玖仟";
            break;
    }
    if (!(i8 || i7 || i6 || i5))
        zero_1_exist = 1;
    if (!zero_1_exist && i10_19_exist && !i8)
    {
        cout << "零";
        zero_1_exist = 1;
    }
    switch (i7) //百万位
    {
        case 1:
            cout << "壹佰";
            break;
        case 2:
            cout << "贰佰";
            break;
        case 3:
            cout << "叁佰";
            break;
        case 4:
            cout << "肆佰";
            break;
        case 5:
            cout << "伍佰";
            break;
        case 6:
            cout << "陆佰";
            break;
        case 7:
            cout << "柒佰";
            break;
        case 8:
            cout << "捌佰";
            break;
        case 9:
            cout << "玖佰";
            break;
    }
    if (!zero_1_exist && !i7 && i6 && num >= 1000000)
    {
        cout << "零";
        zero_1_exist = 1;
    }
    switch (i6) //十万位
    {
        case 1:
            cout << "壹拾";
            break;
        case 2:
            cout << "贰拾";
            break;
        case 3:
            cout << "叁拾";
            break;
        case 4:
            cout << "肆拾";
            break;
        case 5:
            cout << "伍拾";
            break;
        case 6:
            cout << "陆拾";
            break;
        case 7:
            cout << "柒拾";
            break;
        case 8:
            cout << "捌拾";
            break;
        case 9:
            cout << "玖拾";
            break;
    }
    if (i7 && !i6 && i5 && num >= 100000)
    {
        cout << "零";
        zero_1_exist = 1;
    }
    switch (i5) //万位
    {
        case 1:
            cout << "壹";
            break;
        case 2:
            cout << "贰";
            break;
        case 3:
            cout << "叁";
            break;
        case 4:
            cout << "肆";
            break;
        case 5:
            cout << "伍";
            break;
        case 6:
            cout << "陆";
            break;
        case 7:
            cout << "柒";
            break;
        case 8:
            cout << "捌";
            break;
        case 9:
            cout << "玖";
            break;
    }
    if (i5 != 0 || i6 != 0 || i7 != 0 || i8 != 0)
        cout << "万";
    switch (i4) //千位
    {
        case 1:
            cout << "壹仟";
            break;
        case 2:
            cout << "贰仟";
            break;
        case 3:
            cout << "叁仟";
            break;
        case 4:
            cout << "肆仟";
            break;
        case 5:
            cout << "伍仟";
            break;
        case 6:
            cout << "陆仟";
            break;
        case 7:
            cout << "柒仟";
            break;
        case 8:
            cout << "捌仟";
            break;
        case 9:
            cout << "玖仟";
            break;
    }
    if (!(i4 || i3 || i2 || i1))
        zero_2_exist = 1;
    if (!zero_2_exist && !i4 && num >= 1000)
    {
        cout << "零";
        zero_2_exist = 1;
    }
    switch (i3) //百位
    {
        case 1:
            cout << "壹佰";
            break;
        case 2:
            cout << "贰佰";
            break;
        case 3:
            cout << "叁佰";
            break;
        case 4:
            cout << "肆佰";
            break;
        case 5:
            cout << "伍佰";
            break;
        case 6:
            cout << "陆佰";
            break;
        case 7:
            cout << "柒佰";
            break;
        case 8:
            cout << "捌佰";
            break;
        case 9:
            cout << "玖佰";
            break;
    }
    if (!zero_2_exist && !i3 && (i2 || i1) && num >= 100)
    {
        cout << "零";
        zero_2_exist = 1;
    }
    switch (i2) //十位
    {
        case 1:
            cout << "壹拾";
            break;
        case 2:
            cout << "贰拾";
            break;
        case 3:
            cout << "叁拾";
            break;
        case 4:
            cout << "肆拾";
            break;
        case 5:
            cout << "伍拾";
            break;
        case 6:
            cout << "陆拾";
            break;
        case 7:
            cout << "柒拾";
            break;
        case 8:
            cout << "捌拾";
            break;
        case 9:
            cout << "玖拾";
            break;
    }
    if (!i2 && i3 && i1)
    {
        cout << "零";
        zero_2_exist = 1;
    }
    switch (i1) //圆
    {
        case 1:
            cout << "壹";
            break;
        case 2:
            cout << "贰";
            break;
        case 3:
            cout << "叁";
            break;
        case 4:
            cout << "肆";
            break;
        case 5:
            cout << "伍";
            break;
        case 6:
            cout << "陆";
            break;
        case 7:
            cout << "柒";
            break;
        case 8:
            cout << "捌";
            break;
        case 9:
            cout << "玖";
            break;
    }
    if (num >= 1)
        cout << "圆";
    if (i_1 == 0 && i_2 == 0 && num >= 1)
    {
        cout << "整";
        zheng_exist = 1;
    }
    switch (i_1) //角
    {
        case 1:
            cout << "壹角";
            break;
        case 2:
            cout << "贰角";
            break;
        case 3:
            cout << "叁角";
            break;
        case 4:
            cout << "肆角";
            break;
        case 5:
            cout << "伍角";
            break;
        case 6:
            cout << "陆角";
            break;
        case 7:
            cout << "柒角";
            break;
        case 8:
            cout << "捌角";
            break;
        case 9:
            cout << "玖角";
            break;
        default:
            if (i_2 != 0 && num >= 0.1)
                cout << "零";
            break;
    }
    if (i_2 == 0 && num >= 0.1 && !zheng_exist)
        cout << "整";
    switch (i_2) //分
    {
        case 1:
            cout << "壹";
            break;
        case 2:
            cout << "贰";
            break;
        case 3:
            cout << "叁";
            break;
        case 4:
            cout << "肆";
            break;
        case 5:
            cout << "伍";
            break;
        case 6:
            cout << "陆";
            break;
        case 7:
            cout << "柒";
            break;
        case 8:
            cout << "捌";
            break;
        case 9:
            cout << "玖";
            break;
    }
    if (i_2 != 0)
        cout << "分";
    if (num == 0)
        cout << "零";
    cout << endl;
    return 0;
}