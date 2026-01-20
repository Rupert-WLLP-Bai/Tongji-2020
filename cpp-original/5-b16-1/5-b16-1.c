/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include<stdio.h>
#include<math.h>
#include<string.h>
#define SIZE 10

//输入数据
void input(char id[10][8], char name[10][8],  int score[10])
{
	for (int i = 0; i < SIZE; i++)
	{
		printf("请输入第%d个人的学号、姓名、成绩\n", i + 1);
		scanf("%s", &id[i]);
		scanf("%s", &name[i]);
		scanf("%d", &score[i]);
	}
}

//交换名字
void swap_name(char a[8], char b[8])
{
	char t[8];
	for (int i = 0; i < 8; i++)
	{
		t[i] = a[i];
		a[i] = b[i];
		b[i] = t[i];
	}
}

//交换id
void swap_id(char a[8], char b[8])
{
	char t[8];
	for (int i = 0; i < 8; i++)
	{
		t[i] = a[i];
		a[i] = b[i];
		b[i] = t[i];
	}
}

//交换两个成绩的值
void swap_score(int score[10], int i, int j)
{
	int temp = score[i];
	score[i] = score[j];
	score[j] = temp;
}

//比较学号的大小,前比后大返回0，前比后小返回1(需要用swap),相等返回-1
int compare(char a[8], char b[8])
{
	for (int i = 0; i < 8; i++)
	{
		if (a[i] > b[i])
			return 0;
		else if (a[i] < b[i])
			return 1;
		else
			continue;
	}
	return -1;
}

//冒泡排序
void bubble(char id[10][8], char name[10][8], int score[10])
{
	//比较id的顺序并交换值
	for (int i = 0; i < SIZE - 1; i++)
	{
		for (int j = 0; j < SIZE - i - 1; j++)
		{
			if (compare(id[j], id[j + 1]))
			{
				swap_id(id[j], id[j + 1]);
				swap_name(name[j], name[j + 1]);
				swap_score(score, j, j + 1);
			}
		}
	}
}

void print_result(char id[10][8], char name[10][8], int score[10])
{
	printf("\n不及格名单:\n");
	for (int i = 0; i < SIZE; i++)
	{
		if (score[i] < 60)
			printf("%s %s %d\n", id[i], name[i], score[i]);
	}
}

int main()
{
	char name[10][8];
	char id[10][8];
	int score[10];
	input(id, name, score);
	bubble(id, name,score);
	print_result(id, name, score);
	return 0;
}