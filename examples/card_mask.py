import poker_eval_rs

result = poker_eval_rs.string_to_mask("2h4d5s6h7d")
print(result)
result2 = poker_eval_rs.eval_n("2h4d5s6h7d")
print(result2)
result3 = poker_eval_rs.eval_low("2h4d5s6h7d")
print(result3)