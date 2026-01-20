/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
int main()
{
	int x;
	int _return;
	while (1) {
		printf("请输入x的值(0-100)：");
		_return=scanf("%d", &x); //读入x的方式scanf且格式符为%d
		while(_return==0)
		{
			rewind(stdin);
			printf("请输入x的值(0-100)：");
			_return = scanf("%d", &x);
		}
		if (x >= 0 && x <= 100)
			break;
	}

	printf("x=%d\n", x);

	return 0;
}