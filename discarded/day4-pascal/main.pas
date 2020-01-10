program Day4;
var
    currentPassword: longint;
    amountOfPossiblePasswords: longint;
    currentPasswordStr: string;
    passwordCharIndex: integer;
    passwordCharInt: integer;
    lowestChar: integer;
    lastChar: string;
    codeDiscard: integer;
    success: boolean;
begin
    currentPassword := 193651;
    amountOfPossiblePasswords := 0;

    while currentPassword <= 649729 do begin
        success := true;
        str(currentPassword, currentPasswordStr);
        { Okay, weird thing: indexes in Pascal start at 1, apparently. That's kind of akward, but I guess for who would learn it as a first language it would be actually simpler. }
        { writeln('Current password: ', currentPasswordStr); }

        { Check that the digits never decrease... }
        passwordCharIndex := 1; lowestChar := 0;
        while passwordCharIndex <= length(currentPasswordStr) do begin
            val(currentPasswordStr[passwordCharIndex], passwordCharInt, codeDiscard);
            if passwordCharInt < lowestChar then begin
                success := false;
                break;
            end;
            passwordCharIndex := passwordCharIndex + 1;
        end;

        { Check that there are at least two consecutive characters }
        if success then begin
            passwordCharIndex := 1; lastChar := '-1';
            success := false;
            while passwordCharIndex <= length(currentPasswordStr) do begin
                if (lastChar <> '-1') and (currentPasswordStr[passwordCharIndex] = lastChar) then begin
                    success := true;
                    break;
                end;
                lastChar := currentPasswordStr[passwordCharIndex];
                passwordCharIndex := passwordCharIndex + 1;
            end;
        end;

        if success then amountOfPossiblePasswords := amountOfPossiblePasswords + 1;
        currentPassword := currentPassword + 1;
    end;

    writeln('Amount of possible passwords: ', amountOfPossiblePasswords);
end.
