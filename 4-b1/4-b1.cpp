/*2052526 信15 白俊豪*/
#include <iostream>
#include <cmath>
using namespace std;

/* 可根据需要添加相应的内容 */

/***************************************************************************
  函数名称：
  功    能：输出大写的0~9
  输入参数：
  返 回 值：
  说    明：除本函数外，不允许任何函数中输出“零”-“玖”!!!!!!
***************************************************************************/
void daxie(int num, int flag_of_zero)
{
    /* 不允许对本函数做任何修改 */
    switch (num)
    {
        case 0:
            if (flag_of_zero) //此标记什么意思请自行思考
                cout << "零";
            break;
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
        default:
            cout << "error";
            break;
    }
}

/* 可根据需要自定义其它函数(也可以不定义) */

/***************************************************************************
  函数名称：
  功    能：
  输入参数：
  返 回 值：
  说    明：
***************************************************************************/

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

    daxie(i10, 0); //十亿位
    if (i10)
        cout << "拾";

    daxie(i9, 0); //亿位

    if (i10 || i9)
        cout << "亿";
    else
        i10_19_exist = 0;
    daxie(i8, 0); //千万位
    if (!(i8 || i7 || i6 || i5))

        zero_1_exist = 1;
    if (!zero_1_exist && i10_19_exist && !i8)
    {
        daxie(0, 1);
        zero_1_exist = 1;
    }
    if (i8)
        cout << "仟";
    daxie(i7, 0); //百万位

    if (!zero_1_exist && !i7 && i6 && num >= 1000000)
    {
        daxie(0, 1);
        zero_1_exist = 1;
    }
    if (i7)
        cout << "佰";
    daxie(i6, 0); //十万位
    if (i7 && !i6 && i5 && num >= 100000)
    {
        daxie(0, 1);
        zero_1_exist = 1;
    }
    if (i6)
        cout << "拾";
    daxie(i5, 0); //万位

    if (i5 != 0 || i6 != 0 || i7 != 0 || i8 != 0)
        cout << "万";

    daxie(i4, 0); //千位
    if (!(i4 || i3 || i2 || i1))
        zero_2_exist = 1;
    if (!zero_2_exist && !i4 && num >= 1000)
    {
        daxie(0, 1);
        zero_2_exist = 1;
    }
    if (i4)
        cout << "仟";
    daxie(i3, 0); //百位

    if (!zero_2_exist && !i3 && (i2 || i1) && num >= 100)
    {
        daxie(0, 1);
        zero_2_exist = 1;
    }
    if (i3)
        cout << "佰";
    daxie(i2, 0); //十位
    if (!i2 && i3 && i1)
    {
        daxie(0, 1);
        zero_2_exist = 1;
    }
    if (i2)
        cout << "拾";
    daxie(i1, 0); //圆
    if (num >= 1)
        cout << "圆";
    if (i_1 == 0 && i_2 == 0 && num >= 1)
    {
        cout << "整";
        zheng_exist = 1;
    }
    daxie(i_1, 0); //角
    if (i_1 == 0 && i_2 != 0 && num >= 0.1)
        daxie(0, 1);
    if (i_1)
        cout << "角";
    if (i_2 == 0 && num >= 0.1 && !zheng_exist)
        cout << "整";
    daxie(i_2, 0); //分
    if (i_2 != 0)
        cout << "分";
    if (num == 0)
        daxie(0, 1);
    cout << endl;
    return 0;
}