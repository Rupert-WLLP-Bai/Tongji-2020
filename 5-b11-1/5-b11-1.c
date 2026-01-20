/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include<math.h>
//可按需增加需要的头文件

const char chistr[] = "零壹贰叁肆伍陆柒捌玖"; /* 所有输出大写 "零" ~ "玖" 的地方，只允许从这个数组中取值 */
char result[256];  /* 除result外，不再允许定义任何形式的全局变量 */

/* --允许添加需要的函数 --*/
void add_result(int num, int i)
{
	if (num)
	{
		result[i++] = chistr[num*2];
		result[i++] = chistr[num*2 + 1];
	}
}

void print_zero(int i)
{
	result[i++] = chistr[0];
	result[i++] = chistr[1];
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
	/* --允许添加需要的内容 --*/
	char yi[3] = { "亿" };
	char wan[3] = { "万" };
	char qian[3] = { "仟" };
	char bai[3] = { "佰" };
	char shi[3] = { "拾" };
	char yuan[3] = { "圆" };
	char jiao[3] = { "角" };
	char fen[3] = { "分" };
	char zheng[3] = { "整" };
	int k = 0;
	int i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i_1, i_2, temp1, temp2, temp3, temp4, temp5, temp6, temp7, temp8, temp9;
	double temp_1, num, num_int, num_decimal_part;
	int i10_19_exist = 1, zero_1_exist = 0, zero_2_exist = 0, zheng_exist = 0;
	printf("请输入[0-100亿)之间的数字,小数点后最多两位：\n");
	scanf("%lf", &num);
	num_int = num - fmod(num, 1);
	i1 = (int)(fmod(num_int, 10));
	temp1 = (int)(num_int / 10);
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
	i_1 = (int)((num_decimal_part * 10) + 1e-4);
	temp_1 = num_decimal_part * 100 + 1e-3;
	i_2 = (int)(fmod(temp_1, 10) + 1e-4);

	add_result(i10, k);
	if (i10) //十亿位
	{
		k += 2;
		result[k++] = shi[0];
		result[k++] = shi[1];
	}

	add_result(i9, k);
	if (i9)
		k += 2;
	if (i10 != 0 || i9 != 0)
	{
		result[k++] = yi[0];
		result[k++] = yi[1];
	}
	else
		i10_19_exist = 0;

	add_result(i8, k);
	if (i8)//千万位
	{
		k += 2;
		result[k++] = qian[0];
		result[k++] = qian[1];
	}

	if (!(i8 || i7 || i6 || i5))
		zero_1_exist = 1;
	if (!zero_1_exist && i10_19_exist && !i8)
	{
		print_zero(k);
		k += 2;
		zero_1_exist = 1;
	}

	add_result(i7, k);
	if (i7)//百万位
	{
		k += 2;
		result[k++] = bai[0];
		result[k++] = bai[1];
	}

	if (!zero_1_exist && !i7 && i6 && num >= 1000000)
	{
		print_zero(k);
		k += 2;
		zero_1_exist = 1;
	}

	add_result(i6, k);
	if (i6)//十万位
	{
		k += 2;
		result[k++] = shi[0];
		result[k++] = shi[1];
	}
	if (i7 && !i6 && i5 && num >= 100000)
	{
		print_zero(k);
		k += 2;
		zero_1_exist = 1;
	}

	add_result(i5, k);
	if (i5)
		k += 2;
	if (i5 != 0 || i6 != 0 || i7 != 0 || i8 != 0)
	{
		result[k++] = wan[0];
		result[k++] = wan[1];
	}

	add_result(i4, k);
	if (i4)//千位
	{
		k += 2;
		result[k++] = qian[0];
		result[k++] = qian[1];
	}

	if (!(i4 || i3 || i2 || i1))
		zero_2_exist = 1;
	if (!zero_2_exist && !i4 && num >= 1000)
	{
		print_zero(k);
		k += 2;
		zero_2_exist = 1;
	}

	add_result(i3, k);
	if (i3)//百位
	{
		k += 2;
		result[k++] = bai[0];
		result[k++] = bai[1];
	}

	if (!zero_2_exist && !i3 && (i2 || i1) && num >= 100)
	{
		print_zero(k);
		k += 2;
		zero_2_exist = 1;
	}

	add_result(i2, k);
	if (i2)//十位
	{
		k += 2;
		result[k++] = shi[0];
		result[k++] = shi[1];
	}

	if (!i2 && i3 && i1)
	{
		print_zero(k);
		k += 2;
		zero_2_exist = 1;
	}

	add_result(i1, k);//个位
	if (i1)
		k += 2;
	if (num >= 1)
	{
		result[k++] = yuan[0];
		result[k++] = yuan[1];
	}
	if (i_1 == 0 && i_2 == 0 && num >= 1)
	{
		result[k++] = zheng[0];
		result[k++] = zheng[1];
		zheng_exist = 1;
	}

	add_result(i_1, k);
	if (i_1)
	{
		k += 2;
		result[k++] = jiao[0];
		result[k++] = jiao[1];
	}
	if (!i_1)
	{
		if (i_2 != 0 && num >= 0.1)
		{
			print_zero(k);
			k += 2;
		}
	}

	if (i_2 == 0 && num >= 0.1 && !zheng_exist)
	{
		result[k++] = zheng[0];
		result[k++] = zheng[1];
	}

	add_result(i_2, k);

	if (i_2)
	{
		k += 2;
		result[k++] = fen[0];
		result[k++] = fen[1];
	}
	if (num == 0)
	{
		print_zero(k);
		k += 2;
	}

	printf("%s\n", result);  /* 转换得到的大写结果，只允许用本语句输出，之前不允许任何形式的部分输出 */
	return 0;
}
