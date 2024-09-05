// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/submissions/1379985842

package mini33

func maxProfit(prices []int) int {
	profit := 0
	minPrice := prices[0]
	for _, price := range prices {
		minPrice = min(price, minPrice)
		if price-minPrice > 0 {
			profit += price - minPrice
			minPrice = price
		}
	}
	return profit
}
