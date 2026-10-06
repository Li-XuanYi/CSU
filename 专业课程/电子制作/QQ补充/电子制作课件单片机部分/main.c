#include <reg52.h>

unsigned char code LedChar[] = {
    0xC0, 0xF9, 0xA4, 0xB0, 0x99, 0x92, 0x82, 0xF8,
    0x80, 0x90, 0x88, 0x83, 0xC6, 0xA1, 0x86, 0x8E
};
unsigned char code table[] = {0xFE, 0xFD,0xFB, 0xF7,0xEF,0xDF,0xBF,0x7F,0xFE, 0xFD,0xFB, 0xF7,0xEF,0xDF,0xBF,0x7F};

void main()
{
    TMOD = 0x01;  
    TH0  = 0x8A;  
    TL0  = 0xD0;
    TR0  = 1;     
    ET0 = 1;
    EA = 1; 

    while (1);
}
void InterruptTimer0( ) interrupt 1
{
    static unsigned char cnt = 0;  
    static unsigned char sec = 0;  
    TH0 = 0xA8; 
    TL0 = 0xD0; 
     cnt++;           
     if (cnt >= 30)   
        {
                cnt = 0;            
                P0 = LedChar[sec];
                P1 = table[sec];
                sec++;             
                sec &= 0x0F;      
         }
}
