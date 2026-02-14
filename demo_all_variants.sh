#!/bin/bash
CLI="./target/release/poker_eval_rs_cli"

echo "=========================================="
echo "      POKER-EVAL-RS : DEMO VARIANTS       "
echo "=========================================="
echo ""

# Function to run a demo case
run_demo() {
    title="$1"
    match="$2"
    echo "--------------------------------------------------------"
    echo "VARIANT: $title"
    echo "CMD: $match"
    echo ""
    eval "$match"
    echo ""
}

# 1. HOLD'EM
# AsKs vs 2h2d on JsTs5d
run_demo "Texas Hold'em" \
    "$CLI compare 'AsKs' '2h2d' -b 'JsTs5d' -g holdem"

# 2. SHORT DECK
# Flush > Full House logic check
run_demo "Short Deck (Flush vs Full House)" \
    "$CLI compare '9s8s' 'AdAc' -b 'AsKsQsJdJc' -g shortdeck"

# 3. OMAHA HI
# A-K-Q-J vs 2-2-3-3 ds
run_demo "Omaha Hi" \
    "$CLI compare 'AsKsQsJs' '2h2d3h3d' -b 'Th9h5d' -g omaha"

# 4. OMAHA HI/LO (8-or-better)
# Scoop check: A-2-3-4 vs A-K-Q-J on low board
run_demo "Omaha Hi/Lo" \
    "$CLI compare 'As2s3d4d' 'AhKhQhJh' -b '5s6s7s8d9d' -g omaha8"

# 4b. OMAHA 5 (Big O) Hi
run_demo "Omaha 5 (Big O) Hi" \
    "$CLI compare 'AsKsQsJs9s' '2h2d3h3d4h' -b 'Th9h5d' -g omaha5"

# 4c. OMAHA 5 Hi/Lo
run_demo "Omaha 5 Hi/Lo" \
    "$CLI compare 'As2s3d4d5d' 'AhKhQhJhTh' -b '5s6s7s8d9d' -g omaha85"

# 4d. OMAHA 6 Hi
run_demo "Omaha 6 Hi" \
    "$CLI compare 'AsKsQsJs9s8s' '2h2d3h3d4h4d' -b 'Th9h5d' -g omaha6"

# 5. STUD 7-Card
run_demo "7-Card Stud" \
    "$CLI compare 'AsKsQsJsTs9d8d' '2h2d2c3h3d3c4h' -g stud7"

# 6. RAZZ (A-5 Low)
run_demo "Razz (A-5 Low)" \
    "$CLI compare 'As2s3s4s5s6s7s' 'KhQhJhTh9h8h7h' -g razz"

# 7. LOWBALL 2-7
# 2-3-4-5-7 (Wheel) vs Straight
run_demo "Lowball 2-7" \
    "$CLI compare '2s3s4s5s7d' '3s4s5s6s7s' -g lowball27"

# 8. 5-CARD DRAW
run_demo "5-Card Draw" \
    "$CLI compare 'AsKsQsJsTs' '2h2d2c2s9h' -g draw5"

# 9. EQUITY CALCULATION (Omaha Example - Testing Parsed Performance)
run_demo "Equity: Omaha Hi (Monte Carlo)" \
    "$CLI equity 'AsKsQsJs' '2h2d3h3d' -g omaha -m -i 100000"

echo "--------------------------------------------------------"
echo "Demo Completed."
