/*2052526 信15 白俊豪*/
#include<iostream>
#include<string.h>
using namespace std;

int main()
{
	int cnt[5] = { 0 };//计数,3组,5种类型
	char str[3][128];
	for (int i = 0; i < 3; i++)
	{
		cout << "请输入第" << i + 1 << "行" << endl;
		fgets(str[i], 128, stdin);
	}

	for (int i = 0; i < 3; i++)
	{
		for (unsigned int j = 0; j < strlen(str[i]); j++)
		{
			if (str[i][j] >= 'A' && str[i][j] <= 'Z')//大写	
			{
				cnt[0]++;
				continue;
			}
			else if (str[i][j] >= 'a' && str[i][j] <= 'z')//小写
			{
				cnt[1]++;
				continue;
			}
			else if (str[i][j] >= '0' && str[i][j] <= '9')//数字	
			{
				cnt[2]++;
				continue;
			}
			else if (str[i][j] == ' ')//空格	
			{
				cnt[3]++;
				continue;
			}
			else if (str[i][j] < 0XFF && str[i][j]>0X80)//汉字计入其他
			{
				if (str[i][j + 1] > 0X40)
					if (str[i][j + 1] != 0X7F && str[i][j + 1] != 0XFF)
					{
						cnt[4]++;//汉字占两个位置
						j++;
						continue;
					}
			}
			else//除汉字以外的其他字符(滤去换行符)
			{
				if (str[i][j] != '\n')
				{
					cnt[4]++;
					continue;
				}
			}
		}
	}

	cout << "大写 : " << cnt[0] << endl;
	cout << "小写 : " << cnt[1] << endl;
	cout << "数字 : " << cnt[2] << endl;
	cout << "空格 : " << cnt[3] << endl;
	cout << "其他 ：" << cnt[4] << endl;

	return 0;
}