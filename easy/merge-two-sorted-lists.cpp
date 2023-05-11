/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        if(list1 == nullptr) return list2;
        if(list2 == nullptr) return list1;

        ListNode* head = list1;
        ListNode* h1 = list1->next;
        ListNode* h2 = list2;
        if(list1->val > list2->val) {
            head = list2;
            h1 = list1;
            h2 = list2->next;
        }
        ListNode* tail = head;
        while(h1 != nullptr && h2 != nullptr) {
            if(h1->val <= h2->val) {
                // append h1 to tail
                tail->next = h1;
                tail = tail->next;
                h1 = h1->next;
            } else {
                tail->next = h2;
                tail = tail->next;
                h2 = h2->next;
            }
        }

        while(h1 != nullptr) {
            tail->next = h1;
            tail = tail->next;
            h1 = h1->next;
        }
        while(h2 != nullptr) {
            tail->next = h2;
            tail = tail->next;
            h2 = h2->next;
        }
        return head;
    }
};
