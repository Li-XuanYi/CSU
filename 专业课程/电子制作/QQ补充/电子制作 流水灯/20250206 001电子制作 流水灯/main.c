#include <REGX52.H>
#include <intrins.h>


unsigned char tube[20]={0x00,0x3f,0x06,0x5b,0x4f,0x66,0x6d,0x7d,0x07,0x7f,0x6f};
void Delay500ms(void)	//@12.000MHz
{
	unsigned char data i, j, k;

	_nop_();
	i = 4;
	j = 205;
	k = 187;
	do
	{
		do
		{
			while (--k);
		} while (--j);
	} while (--i);
}


int main()
{
	while(1)
	{
		int i=0;
		for(i=0;i<8;i++)
		{
			P1=~(0x01<<i);
			P0=~tube[i+2];
			Delay500ms();
		}
	}
}
