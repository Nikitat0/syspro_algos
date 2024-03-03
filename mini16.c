// https://leetcode.com/problems/linked-list-cycle-ii/submissions/1191482998

#include <stddef.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *detectCycle(struct ListNode *head) {
  if (head == NULL)
    return NULL;

  struct ListNode *i = head, *j = head;
  do {
    i = i->next;
    j = j->next;
    if (j == NULL)
      return NULL;
    j = j->next;
    if (j == NULL)
      return NULL;
  } while (i != j);
  i = head;
  while (i != j) {
    i = i->next;
    j = j->next;
  }
  return i;
}
