/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;

int main()
{
	int i;
	int score[1000];
	int empty = 1;
	for (i = 0; i < 1000; i++)//初始化
	{
		score[i] = -1;
	}
	//读入数据
	i = 0;
	cout << "请输入成绩(最多1000个),以-1结尾 : " << endl;
	while (i < 1000)//读入数据(最多1000个)
	{
		int b;
		cin >> b;
		if (b < 0)//负数退出
			break;
		score[i] = b;
		i++;
		empty = 0;
	}
	if (!empty) {
		cout << "输入的数组为 : ";
		for (i = 0; i < 1000; i++)
		{
			if (score[i] == -1)
				break;
			if (i % 10 == 0)
				cout << endl;
			cout << score[i] << " ";
		}

		cout << endl << "分数与名次的对应关系为 : " << endl;

		int count[101];
		for (i = 0; i < 101; i++)
			count[i] = 0;

		for (int s = 100; s >= 0; s--)
		{
			for (i = 0; i < 1000; i++)
			{
				if (score[i] == s)
					count[s]++;
			}
		}

		//排序
		int rank = 1;
		for (int i = 0; i < 1000; i++)
		{
			for (int j = 0; j < 1000 - 1 - i; j++)
			{
				if (score[j] < score[j + 1])
				{
					int temp = score[j];
					score[j] = score[j + 1];
					score[j + 1] = temp;
				}
			}
		}

		for (i = 0; i < 1000; i++)
		{
			if (score[i] == -1)
				break;
			cout << score[i] << " " << rank << endl;
			if (score[i + 1] != score[i])
				rank += count[score[i]];
		}
	}
	else
	{
		cout << "无有效输入" << endl;
	}
	return 0;
}