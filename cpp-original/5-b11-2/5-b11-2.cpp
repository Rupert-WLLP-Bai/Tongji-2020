/*2052526 信15 白俊豪*/
#include <iostream>
#include<cmath>
#include <string>
//可按需增加需要的头文件
using namespace std;

const char chistr[] = "零壹贰叁肆伍陆柒捌玖"; /* 所有输出大写 "零" ~ "玖" 的地方，只允许从这个数组中取值 */
string result;  /* 除result外，不再允许定义任何形式的全局变量 */

/* --允许添加需要的函数 --*/


void add_result(int num)
{
	if (num)
	{
		result += chistr[num * 2];
		result += chistr[num * 2 + 1];
	}
}
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

	add_result(i10);
	if (i10) //十亿位
		result += "拾";
	add_result(i9);
	if (i10 != 0 || i9 != 0)
		result += "亿";
	else
		i10_19_exist = 0;

	add_result(i8);
	if (i8)//千万位
		result += "仟";

	if (!(i8 || i7 || i6 || i5))
		zero_1_exist = 1;
	if (!zero_1_exist && i10_19_exist && !i8)
	{
		result += chistr[0];
		result += chistr[1];
		zero_1_exist = 1;
	}

	add_result(i7);
	if (i7)//百万位
		result += "佰";

	if (!zero_1_exist && !i7 && i6 && num >= 1000000)
	{
		result += chistr[0];
		result += chistr[1];
		zero_1_exist = 1;
	}

	add_result(i6);
	if (i6)//十万位
		result += "拾";
	if (i7 && !i6 && i5 && num >= 100000)
	{
		result += chistr[0];
		result += chistr[1];
		zero_1_exist = 1;
	}

	add_result(i5);
	if (i5 != 0 || i6 != 0 || i7 != 0 || i8 != 0)
		result += "万";

	add_result(i4);
	if (i4)//千位
		result += "仟";

	if (!(i4 || i3 || i2 || i1))
		zero_2_exist = 1;
	if (!zero_2_exist && !i4 && num >= 1000)
	{
		result += chistr[0];
		result += chistr[1];
		zero_2_exist = 1;
	}

	add_result(i3);
	if (i3)//百位
		result += "佰";

	if (!zero_2_exist && !i3 && (i2 || i1) && num >= 100)
	{
		result += chistr[0];
		result += chistr[1];
		zero_2_exist = 1;
	}

	add_result(i2);
	if (i2)//十位
		result += "拾";

	if (!i2 && i3 && i1)
	{
		result += chistr[0];
		result += chistr[1];
		zero_2_exist = 1;
	}

	add_result(i1);//个位


	if (num >= 1)
		result += "圆";
	if (i_1 == 0 && i_2 == 0 && num >= 1)
	{
		result += "整";
		zheng_exist = 1;
	}

	add_result(i_1);
	if (i_1)
		result += "角";
	if (!i_1)
	{
		if (i_2 != 0 && num >= 0.1)
		{
			result += chistr[0];
			result += chistr[1];
		}
	}

	if (i_2 == 0 && num >= 0.1 && !zheng_exist)
		result += "整";

	add_result(i_2);

	if (i_2 != 0)
		result+= "分";
	if (num == 0)
	{
		result += chistr[0];
		result += chistr[1];
	}
	
	cout << result << endl;
	return 0;
}