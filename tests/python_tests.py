
import sys
import os
import unittest

# Try to import the module. In a real environment, this would require 
# 'maturin develop' or setting PYTHONPATH to the build artifact.
# Here we mock it or expect it to fail if not built, but we provide the test structure.
try:
    import poker_eval_rs
except ImportError:
    print("poker_eval_rs module not found. Tests will fail if not verified manually.")
    poker_eval_rs = None

class TestPokerEval(unittest.TestCase):
    def setUp(self):
        if poker_eval_rs is None:
            self.skipTest("poker_eval_rs module not available")

    def test_card_creation(self):
        c = poker_eval_rs.Card("As")
        self.assertEqual(c.rank, 12) # Ace
        self.assertEqual(c.suit, 3) # Spades
        self.assertEqual(str(c), "As")
        
        c2 = poker_eval_rs.Card.from_id(0)
        self.assertEqual(str(c2), "2h") # 2h is 0

    def test_hand_creation(self):
        h = poker_eval_rs.Hand("As Ks")
        self.assertEqual(len(h), 2)
        self.assertEqual(str(h), "Ks As") # Order might vary depending on impl, usually sorted by rank
        
        h.add("Qd")
        self.assertEqual(len(h), 3)
        self.assertTrue("Qd" in str(h))

    def test_hand_duplicate(self):
        h = poker_eval_rs.Hand("As")
        with self.assertRaises(ValueError):
            h.add("As")

    def test_eval_n(self):
        # Royal Flush
        res = poker_eval_rs.eval_n("As Ks Qs Js Ts")
        self.assertEqual(res, "StFlush")
        
        # Pairs
        res = poker_eval_rs.eval_n("As Ad 2d 3c 4s")
        self.assertEqual(res, "Pair")

    def test_board_texture(self):
        # Rainbow
        bt = poker_eval_rs.BoardTexture.analyze("As Kd Qc")
        self.assertTrue(bt.is_rainbow)
        self.assertFalse(bt.is_monotone)
        
        # Monotone
        bt = poker_eval_rs.BoardTexture.analyze("As Ks Qs")
        self.assertTrue(bt.is_monotone)
        self.assertFalse(bt.is_rainbow)
        
        # Paired
        bt = poker_eval_rs.BoardTexture.analyze("As Ad Ks")
        self.assertTrue(bt.is_paired)

    def test_calculate_outs(self):
        # Flush draw
        # Pocket: As 2s
        # Board: Ks Qs 2d
        # Outs for Flush: any spade (9 remaining)
        outs = poker_eval_rs.calculate_outs("As 2s", "Ks Qs 2d")
        self.assertIn("Flush", outs)
        self.assertEqual(len(outs["Flush"]), 9) # 13 - 4 = 9 spades

    def test_hand_range(self):
        r = poker_eval_rs.HandRange("AKs")
        self.assertEqual(len(r), 4) # AsKs, AdKd, AcKc, AhKh
        
        r2 = poker_eval_rs.HandRange("AA")
        self.assertEqual(len(r2), 6) # 6 combos of AA

    def test_equity_calculator(self):
        # AA vs KK
        range1 = poker_eval_rs.HandRange("AA")
        range2 = poker_eval_rs.HandRange("KK")
        res = poker_eval_rs.calculate_range_equity(range1, range2, "", 1000)
        self.assertTrue(res["equity"] > 80.0) # AA is approx 82% vs KK

    def test_calculate_equity(self):
        # AA vs KK
        res = poker_eval_rs.calculate_equity(["AsAc", "KsKc"], "", "", "holdem", True, 10000)
        players = res["players"]
        self.assertEqual(len(players), 2)
        
        # Check that probabilities sum to 100%
        for p in players:
            total_prob = p["win"] + p["tie"] + p["lose"]
            self.assertTrue(abs(total_prob - 100.0) < 0.01, f"Probabilities should sum to 100, got {total_prob}")
            
        # AA should be favorite (approx 82%)
        aa_stats = next(p for p in players if "AsAc" in p["hand"])
        self.assertTrue(aa_stats["win"] > 80.0)

if __name__ == '__main__':
    unittest.main()
