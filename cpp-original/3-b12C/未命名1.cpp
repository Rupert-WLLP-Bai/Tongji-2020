#include<iostream>
#include<stdlib.h>
#include<time.h>
using namespace std;
int main()
{
	int a[10],i,max,b,min,aver;
	b=0;
	i=0;
	aver=0;
	srand(time(NULL));
	for(i=0;i<=9;i++)
	{
		
		a[i]=rand()%71+30;
		cout<<a[i]<<endl;
		max=(b>=a[i])?b:a[i];
	    min=(b<=a[i])?b:a[i];
		aver+=a[i];
		b=a[i];
	}
	aver=aver/10.0;
	 cout<<"最大值"<<max<<"最小值"<<min<<"平均值"<<aver<<endl;
	 system("pause");
	 return 0;
}

