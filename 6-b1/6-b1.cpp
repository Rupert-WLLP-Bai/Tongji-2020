/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;

#define  N  10	/* 假设最多转换10个数字 */

/* 不允许再定义其它函数、全局变量 */

int main()
{
	/* 如果有不需要的变量，允许删除，但不允许添加或替换为其它类型的变量 */
	char str[256], * p;
	int  a[N] = { 0 }, * pnum, * pa;

	/* 上面的定义不准动(删除不需要的变量除外)，下面为程序的具体实现，要求不得再定义任何变量、常量、常变量 */
	p = str;
	pa = a;
	pnum = a;
	cout << "请输入间隔含有若干正负数字的字符串" << endl;

	/*条件编译*/
#if(__GNUC__)
	gets(str);
#elif(_MSC_VER)
	gets_s(str, 256);
#endif

	
		while (*p != '\0')
		{
			if (pa - pnum >= 10)
				break;
			if (*p >= '0' && *p <= '9')
			{
				*pa = *pa * 10 + (*p - '0');
				p++;

				if (!(*p >= '0' && *p <= '9'))
				{
					pa++;
				}
				continue;
			}
			p++;
		}


	cout << "字符串中包含的整数个数为：" << pa - pnum << "个" << endl;
	cout << "这些整数为:" << endl;

	
	for (int i = 0; i < pa - pnum; pnum++)
	{
		cout << *pnum << " ";
	}
	return 0;
}

