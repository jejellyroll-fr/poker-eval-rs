import pytest
import poker_eval_rs

def test_string_to_mask():
    result = poker_eval_rs.string_to_mask("As Ks")
    # Mask for As (2^51) | Ks (2^50)
    # 51: 0, 50: 1, ...
    # Wait, mask is u64.
    # As is rank 12, suit 3. 12 + 3*13 = 12 + 39 = 51.
    # Ks is rank 11, suit 3. 11 + 39 = 50.
    # Result string is binary.
    assert result != ""

def test_eval_n():
    hand = "As Ks Qs Js Ts"
    val = poker_eval_rs.eval_n(hand)
    assert "Straight Flush" in val

def test_batch_eval():
    hands = ["As Ks Qs Js Ts", "2h 3h 4h 5h 7h"]
    results = poker_eval_rs.eval_n_batch(hands)
    assert len(results) == 2
    assert "Straight Flush" in results[0]
    assert "Flush" in results[1]

def test_omaha_hi():
    hole = "As Ks Qd Jd"
    board = "Ts 9s 2h 3c 4d"
    # As Ks Ts 9s some_other is Flush? No, board has Ts 9s. Hole has As Ks. 4 spades. No flush.
    # Wait, As Ks (hole) + Ts 9s (board) = 4 spades.
    # Need 3 board cards.
    # Board: Ts 9s 2h 3c 4d.
    # Two spades on board. Two spades in hand.
    # Omaha High needs EXACTLY 2 from hole, 3 from board.
    # Hand: As Ks. Board: Ts 9s 2h 3c 4d.
    # 2 hole spades + 2 board spades = 4 spades. No flush.
    # Straight? Q J T 9 8? No 8.
    # Pairs? No.
    # High card Ace.
    val = poker_eval_rs.eval_omaha_hi(hole, board)
    print(val)
    assert "High Card" in val or "Pair" in val # Example check

def test_omaha_hi_lo():
    hole = "As 2s 3d 4d"
    board = "5s 6s 7s 8d 9d"
    # Low potential: A-2-3-4-5-6-7-8.
    # Best Low: A-2-5-6-7? No, must use 2 hole, 3 board.
    # Hole: A, 2. Board: 5, 6, 7. -> A-2-5-6-7. Valid 8-low.
    # Result is tuple (hi, lo)
    hi, lo = poker_eval_rs.eval_omaha_hi_lo(hole, board)
    assert hi != "No valid hand"
    assert lo != "No valid low"

def test_duplicates():
    with pytest.raises(ValueError):
        poker_eval_rs.string_to_mask("As As")

def test_equity_holdem():
    hands = ["As Ks", "2h 2d"]
    res = poker_eval_rs.calculate_equity(hands, game="holdem", monte_carlo=True, iterations=1000)
    assert "players" in res
    assert len(res["players"]) == 2

def test_equity_omaha_hi_lo():
    hands = ["As 2s 3d 4d", "Ks Kd 5h 6h"]
    res = poker_eval_rs.calculate_equity(hands, game="omaha8", monte_carlo=True, iterations=1000)
    assert "players" in res
    p1 = res["players"][0]
    # Check for scoop/lo keys
    assert "scoop" in p1
    assert "win_lo" in p1

def test_equity_probability_sum():
    """Validate that win + tie + lose probabilities sum to ~100% for each player."""
    hands = ["As Ks", "2h 2d"]
    res = poker_eval_rs.calculate_equity(hands, game="holdem", monte_carlo=True, iterations=10000)
    assert "players" in res
    for i, p in enumerate(res["players"]):
        total = p["win"] + p["tie"] + p["lose"]
        assert abs(total - 100.0) < 1.0, (
            f"Player {i}: probabilities sum to {total}, expected ~100"
        )
