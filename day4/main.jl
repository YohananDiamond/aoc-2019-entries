#=
1. It is a six-digit number.
2. The value is within the range given in your puzzle input.
3. Two adjacent digits are the same (like 22 in 122345).
4. Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
Your puzzle input is 193651-649729.
=#

NUMBERS_RANGE = 193651:649729

function digits_at_least_one_repetition(number::Int)::Bool
    number_string = string(number)
    last_digit = "empty"

    for char in number_string
        char_str = string(char)
        if char_str == last_digit
            return true
        end
        last_digit = char_str
    end

    return false
end

function digits_never_decrease(number::Int)::Bool
    number_string = string(number)
    lowest = 0

    for char in number_string
        if parse(Int, char) < lowest
            return false
        end
        lowest = parse(Int, char)
    end

    return true
end

nc = 0
for password in NUMBERS_RANGE
    global nc += Int(digits_at_least_one_repetition(password) && digits_never_decrease(password))
end
println("Possible passwords from part 1: ", nc)

function digits_new_repetition(number::Int)::Bool
    number_string = string(number)
    last_digit = "empty"
    repetition_count = 0

    for char in number_string
        char_str = string(char)
        if char_str == last_digit
            repetition_count += 1
        else
            if repetition_count == 1 # 1 Repetition, 2 equal characters
                return true
            end
            repetition_count = 0
        end
        last_digit = char_str
    end

    # Check at the end of the loop
    repetition_count == 1 ? true : false
end

nc = 0
for password in NUMBERS_RANGE
    global nc += Int(digits_new_repetition(password) && digits_never_decrease(password))
end
println("Possible passwords from part 2: ", nc)

