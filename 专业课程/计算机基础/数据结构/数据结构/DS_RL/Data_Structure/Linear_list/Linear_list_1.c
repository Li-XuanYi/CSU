// 链表实现
#include<stdio.h>
#include<stdlib.h>

typedef struct ListNode
{
    int value;
    struct ListNode* next;
}LN;

LN* newListNode(int val)   // 增添节点
{
    LN* node;
    node = (LN*)malloc(sizeof(LN));
    node->next = NULL;
    node->value = val;
    return node;
}

void insert(LN* n0,LN* P) //插入元素P
{
    P->next = n0->next;
    n0->next = P;
}

void remove(LN* n0) //删除节点n0之后的第一个节点 (n0->P)
{
    if(!n0->next)
    {return;}
    LN* P = n0->next;
    n0->next = P->next;
    free(P);
}

LN* find(LN* head,int index)
{
    for(int i=0;i<index;i++)
    {
        if(head == NULL)
        return NULL;

        head = head->next;
    }
    return head;
}

int main()
{
    LN* n0 = newListNode(12);
    LN* n1 = newListNode(10); 
    LN* n2 = newListNode(18);
    LN* n3 = newListNode(13);
    n0->next = n1;
    n1->next = n2;
    n2->next = n3;

    return 0;
}
