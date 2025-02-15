The following is a description of the language grammar. It will be updated along with the compiler.

## Tokens

$\text{Keywords (including primitive types)}$
$$
\begin{align*}
\textcolor{green}{\text{Int}} &\to \text{int}
\\
\textcolor{green}{\text{Ret}} &\to \text{return}
\end{align*}
$$
$\text{Symbols}$
$$
\begin{align*}
\textcolor{green}{\text{LP}} &\to \text{(}
\\
\textcolor{green}{\text{RP}} &\to \text{)}
\\
\textcolor{green}{\text{LB}} &\to \text{\\\{}
\\
\textcolor{green}{\text{RB}} &\to \text{\\\}}
\\
\textcolor{green}{\text{Semi}} &\to \text{;}
\\
\textcolor{green}{\text{Colon}} &\to \text{:}
\\
\textcolor{green}{\text{Assign}} &\to \text{=}
\end{align*}
$$
$\text{Complex Tokens}$
$$
\begin{align*}
\textcolor{green}{\text{Id}} &\to \text{[\_a-zA-Z][\_a-zA-Z0-9]*}
\\
\textcolor{green}{\text{IntLit}} &\to \text{[0-9]}+
\end{align*}
$$
$\text{Special Tokens (Not meant for parsing)}$
$$
\begin{align*}
\textcolor{green}{\text{EOF}} &\to \text{End-Of-File}
\\
\textcolor{green}{\text{UNKNOWN}} &\to \text{.*}
\end{align*}
$$

## Grammar (So Far)
$$
\begin{align*}
\text{lit} &\to \text{\textcolor{green}{IntLit}}
\\
\text{expr} &\to \text{lit}
\\
&~~~|~~\text{ident}
\\
\text{return} &\to \text{\textcolor{green}{Ret} expr}
\\
\text{stmt} &\to \text{return \textcolor{green}{Semi}}
\\
\text{block} &\to \text{\textcolor{green}{LB} stmt* \textcolor{green}{RB}}
\\
\text{type} &\to \text{\textcolor{green}{Int}}
\\
\text{ident} &\to \text{\textcolor{green}{Id}}
\\
\text{func} &\to \text{ident \textcolor{green}{LP} \textcolor{green}{RP} \textcolor{green}{Colon} type \textcolor{green}{Assign} block}
\\
\text{prog} &\to \text{func* \textcolor{green}{EOF}}
\end{align*}
$$