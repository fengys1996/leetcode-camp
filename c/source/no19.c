#include <stdio.h>

// todo: add unit test
int main(int argc, char **argv)
{
    printf("leetcode no19\n");
}

struct ListNode
{
    int val;
    struct ListNode *next;
};

struct ListNode *removeNthFromEnd(struct ListNode *head, int n)
{
    struct ListNode emptyHead =
    {
        0,
        head
    };
    
    struct ListNode *p2n = NULL;
    struct ListNode *prevOP2n = NULL;
    struct ListNode *p = &emptyHead;
    int num = 1;

    while (p != NULL)
    {
        if (num == n)
            p2n = &emptyHead;
        if (num > n)
        {
            prevOP2n = p2n;
            p2n = p2n->next;
        }
        num++;
        p = p->next;
    }

    if (p2n == NULL)
        return head;

    prevOP2n->next = p2n->next;

    return emptyHead.next;
}