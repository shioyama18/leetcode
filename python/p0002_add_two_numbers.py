class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        p = head = ListNode(0)
        carry = 0

        while l1 or l2:
            # calculate sum
            x = l1.val if l1 else 0
            y = l2.val if l2 else 0
            total = x + y + carry
            total, carry = total % 10, total // 10
            p.next = ListNode(total)

            # proceed to next node
            p = p.next
            l1 = l1.next if l1 else None
            l2 = l2.next if l2 else None

        if carry:
            p.next = ListNode(1)

        return head.next or head
