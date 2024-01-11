import argparse
import poker_eval_rs

def main():
    parser = argparse.ArgumentParser(description="Poker Hand Evaluator")
    parser.add_argument("cards", help="Enter the cards in the format '2h4d5s6h7d'")
    parser.add_argument("-hi", "--high", help="Evaluate the high hand", action="store_true")
    parser.add_argument("-lo", "--low", help="Evaluate the low hand", action="store_true")
    parser.add_argument("-hilo", "--highlow", help="Evaluate both high and low hands", action="store_true")

    args = parser.parse_args()

    if args.high:
        result_high = poker_eval_rs.eval_n(args.cards)
        print(f"High Hand: {result_high}")

    if args.low:
        result_low = poker_eval_rs.eval_low(args.cards)
        print(f"Low Hand: {result_low}")

    if args.highlow:
        result_high = poker_eval_rs.eval_n(args.cards)
        result_low = poker_eval_rs.eval_low(args.cards)
        print(f"High Hand: {result_high}, Low Hand: {result_low}")

if __name__ == "__main__":
    main()
