### **Product Requirement Document (PRD): SS PDV LITE**

| **Documento:** | PRD - SS PDV LITE      |
|:---------------|:-----------------------|
| **Versão:**    | 1.0                    |
| **Data:**      | 12 de setembro de 2025 |
| **Autor:**     | Saed Silva Sousa       |
| **Status:**    | Rascunho               |

### 1. Visão Geral do Projeto

O projeto "SS PDV LITE" visa o desenvolvimento de um software de Ponto de Venda (PDV) projetado especificamente para as
necessidades de pequenas lojas de variedades. O mercado atual oferece soluções complexas e caras, inadequadas para micro
e pequenos empreendedores. Este sistema se concentrará em ser **simples, intuitivo e robusto**, permitindo que o
operador de caixa realize vendas de forma ágil, que o proprietário gerencie o estoque de forma descomplicada e que tenha
acesso a relatórios gerenciais básicos para a tomada de decisões.

O produto resolverá os principais desafios diários do pequeno varejista: lentidão no atendimento, falta de controle
sobre o que entra e sai do estoque e ausência de dados claros sobre o desempenho das vendas.

**Público-Alvo:** Proprietários e funcionários de pequenas lojas de varejo, como lojas de presentes, bazares, pequenas
mercearias e lojas de variedades em geral.

### 2. Objetivos e Metas

* **Objetivo de Negócio:** Aumentar a eficiência operacional da loja, reduzir erros manuais no fechamento de caixa e no
  controle de estoque, e fornecer insights básicos de vendas para o proprietário.
* **Objetivo do Produto:** Entregar um sistema de PDV funcional com os módulos essenciais de Vendas, Estoque e
  Relatórios, que exija o mínimo de treinamento para ser operado.

### 3. Requisitos Funcionais

#### 3.1. Módulo de Vendas (Frente de Caixa)

* **RF001 - Registro de Vendas:**
    * A interface principal deve permitir a adição de produtos à venda.
    * Deve ser possível adicionar um produto via leitura de código de barras.
    * Deve ser possível buscar um produto pelo nome caso não tenha código de barras ou o leitor falhe.
    * Deve ser possível alterar a quantidade de um item na venda.
    * O sistema deve exibir a lista de itens, seus preços e o subtotal em tempo real.

* **RF002 - Finalização de Venda:**
    * O caixa deve poder selecionar o método de pagamento.
    * Suporte para os seguintes métodos: Dinheiro, Cartão de Crédito, Cartão de Débito e PIX.
    * Para pagamentos em dinheiro, o sistema deve calcular o troco automaticamente após o caixa inserir o valor
      recebido.

* **RF003 - Cancelamento e Devolução:**
    * Funcionalidade para cancelar a venda atual em andamento.
    * Funcionalidade para cancelar a última venda finalizada (requer permissão de gerente/proprietário).
    * Funcionalidade para registrar a devolução de um item específico de uma venda passada. O estoque do item devolvido
      deve ser ajustado. O sistema deve solicitar um motivo para a devolução (e.g., "produto com defeito", "desistência
      do cliente").

* **RF004 - Emissão de Recibo:**
    * Após a finalização da venda, o sistema deve oferecer a opção de imprimir um recibo não fiscal em uma impressora
      térmica.
    * O recibo deve conter: nome da loja, data/hora, itens, quantidades, preços unitários, total da venda e forma de
      pagamento.
    * **Nota:** A integração com a SEFAZ para emissão de Nota Fiscal do Consumidor Eletrônica (NFC-e) será considerada
      em uma versão futura.

#### 3.2. Módulo de Cadastro e Estoque

* **RF005 - Cadastro de Produtos:**
    * Tela dedicada para cadastrar, editar e excluir produtos.
    * Campos obrigatórios: Nome do Produto, Preço de Venda.
    * Campos opcionais: Código de Barras (EAN), Estoque Inicial, Estoque Mínimo para Alerta, Unidade de Medida (unidade,
      kg, pacote, etc.).

* **RF006 - Controle de Estoque:**
    * O estoque de um produto deve ser decrementado automaticamente a cada venda registrada.
    * O sistema deve gerar um alerta visual (e.g., na lista de produtos ou em um relatório específico) quando a
      quantidade de um produto atingir o "Estoque Mínimo" definido em seu cadastro.

* **RF007 - Inventário de Estoque:**
    * Funcionalidade que permite ao proprietário realizar a contagem física dos produtos e ajustar a quantidade
      registrada no sistema para refletir a realidade.

#### 3.3. Módulo de Relatórios

* **RF008 - Relatório de Vendas Diárias:**
    * Deve exibir um resumo das vendas para o dia atual ou uma data selecionada.
    * Informações a serem exibidas: Total Bruto Vendido, Total de Descontos (se aplicável), Total Líquido, Ticket Médio,
      Quantidade de Vendas e Quantidade Total de Itens Vendidos.

* **RF009 - Relatório de Produtos Mais Vendidos:**
    * O usuário deve poder selecionar um período (e.g., "Hoje", "Últimos 7 dias", "Este Mês").
    * O sistema deve exibir uma lista dos produtos mais vendidos (em quantidade) no período selecionado.

* **RF010 - Relatório de Fechamento de Caixa:**
    * Função para ser executada ao final do expediente.
    * O relatório deve detalhar o total de vendas por forma de pagamento (Total em Dinheiro, Total em Cartão, etc.).
    * Deve exibir um resumo de todas as transações, incluindo vendas, cancelamentos e devoluções.

### 4. Requisitos Não Funcionais

* **RNF01 - Usabilidade:** A interface deve ser limpa, com poucos elementos na tela de venda para evitar distrações.
  Botões e campos de texto devem ser grandes e legíveis. Ícones devem ser usados para representar ações comuns (
  adicionar, remover, pagar). A curva de aprendizado deve ser mínima.
* **RNF02 - Performance:** A busca de produtos por nome ou código de barras deve retornar resultados em menos de 1
  segundo. O processo de finalização da venda e impressão do recibo deve ser concluído em menos de 3 segundos.
* **RNF03 - Confiabilidade:** O sistema deve ser estável e não travar durante as operações de venda. Em caso de falha de
  energia, os dados da venda em andamento devem ser recuperáveis.
* **RNF04 - Segurança:** Acesso a funções críticas como ajuste de estoque, cancelamento de vendas finalizadas e
  visualização de relatórios financeiros deve ser protegido por uma senha de administrador/proprietário.
* **RNF05 - Compatibilidade:** O software deve ser compatível com o sistema operacional Windows 10 (ou superior),
  leitores de código de barras USB padrão e impressoras térmicas de 80mm.

### 5. Fluxos de Trabalho (Histórias de Usuário)

* **Cenário de Venda:** "Como um **caixa**, eu quero buscar um produto pelo código de barras e adicioná-lo à venda para
  agilizar o atendimento ao cliente."
* **Cenário de Pagamento:** "Como um **caixa**, eu quero selecionar 'Dinheiro' como forma de pagamento, inserir o valor
  recebido e ver o troco calculado automaticamente para evitar erros."
* **Cenário de Fechamento de Caixa:** "Como um **proprietário**, eu quero fechar o caixa no final do dia para conferir o
  total de vendas por método de pagamento e garantir que o valor em dinheiro bate com o registrado no sistema."
* **Cenário de Cadastro de Produto:** "Como um **proprietário**, eu quero cadastrar um novo produto de forma simples,
  informando apenas o nome e o preço, para começar a vendê-lo imediatamente."
* **Cenário de Controle de Estoque:** "Como um **proprietário**, eu quero ser alertado quando um produto popular está
  com estoque baixo para que eu possa fazer um novo pedido a tempo."
* **Cenário de Devolução:** "Como um **caixa**, eu quero registrar a devolução de um item para que o valor seja abatido
  do caixa e o produto retorne ao estoque corretamente."

### 6. Considerações Adicionais

#### 6.1. Escopo do Projeto (Versão 1.0)

* **Dentro do Escopo:** Todas as funcionalidades descritas nos módulos de Vendas, Cadastro/Estoque e Relatórios.
* **Fora do Escopo:** Módulo financeiro completo (contas a pagar/receber), controle de compras de fornecedores, programa
  de fidelidade de clientes, integração com e-commerce, emissão de notas fiscais (NF-e/NFC-e), gestão de múltiplos
  caixas ou lojas.

#### 6.2. Suposições

* A loja já possui o hardware necessário: um computador (desktop ou notebook), um leitor de código de barras USB e uma
  impressora térmica de recibos.
* Os operadores de caixa possuem conhecimento básico de informática.
* O sistema operará em um único computador por loja.

#### 6.3. Restrições

* O desenvolvimento inicial será focado em uma aplicação desktop para Windows.
* O sistema não terá funcionalidades online ou baseadas em nuvem na primeira versão.
* O banco de dados será local, armazenado na máquina onde o sistema está instalado.

### 7. Roadmap Futuro (Pós-lançamento da v1.0)

* **Versão 1.1:** Módulo de Fidelidade de Clientes (cadastro de clientes e histórico de compras).
* **Versão 1.2:** Integração com terminais de pagamento (TEF) para automatizar a transação com cartão.
* **Versão 1.5:** Módulo de Compras (cadastro de fornecedores e registro de notas de entrada de mercadorias).
* **Versão 2.0:** Integração com a SEFAZ para emissão de NFC-e.

---