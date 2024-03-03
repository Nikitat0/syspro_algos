// https://leetcode.com/problems/reverse-linked-list-ii/submissions/1192613314

struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *reverseBetween(struct ListNode *head, int left, int right) {
  if (left == right)
    return head;
  if (left == 1) {
    struct ListNode h;
    h.next = head;
    reverseBetween(&h, left + 1, right + 1);
    return h.next;
  }

  struct ListNode *i = head;
  for (int c = 2; c < left; c++)
    i = i->next;
  struct ListNode *j = i->next;
  struct ListNode *k = j->next;
  for (int c = 0; c < (right - left); c++) {
    struct ListNode *t = k->next;
    k->next = i->next;
    i->next = k;
    k = t;
  }
  j->next = k;
  return head;
}
