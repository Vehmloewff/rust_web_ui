use rust_web_ui::*;

pub struct Example24;

pub struct Example24Props {}

impl Default for Example24Props {
	fn default() -> Example24Props {
		Example24Props { }
	}
}

impl Widget<'_> for Example24 {
	type Props = Example24Props;

	fn render(mut ctx: Ctx<'_>, props: Example24Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::PaddingTop(Size::Exact(24))]);
				
				props.child("1", Dynamic).run("nav", |mut props| {
					props.set_attribute("aria-label", "Breadcrumb");
					
					props.child("1", Dynamic).run("ol", |mut props| {
						props.set_attribute("role", "list");
						props.styles(&[Style::Noop("mx-auto"), Style::Flex, Style::Noop("max-w-2xl"), Style::ItemsCenter, Style::SpaceX(Size::Exact(8)), Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
						
						props.child("1", Dynamic).run("li", |mut props| {
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::MarginRight(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Men"));
								});
								props.child("3", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(16)), Style::TextColor(Color::Fg(33))]);
								});
							});
						});
						props.child("3", Dynamic).run("li", |mut props| {
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::MarginRight(Size::Exact(8)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Clothing"));
								});
								props.child("3", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(16)), Style::TextColor(Color::Fg(33))]);
								});
							});
						});
						props.child("5", Dynamic).run("li", |mut props| {
							props.styles(&[Style::Noop("text-sm")]);
							
							props.child("1", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.set_attribute("aria-current", "page");
								props.styles(&[Style::Noop("font-medium"), Style::TextColor(Color::Fg(56)), Style::OnHover(&[Style::TextColor(Color::Fg(67))])]);
								
								props.child("0", Label).run(|props| props.text("Basic Tee 6-Pack"));
							});
						});
					});
				});
				//  Image gallery 
				props.child("5", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::MarginTop(Size::Exact(24)), Style::Noop("max-w-2xl"), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::Noop("grid")]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-8")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("aspect-h-4"), Style::Noop("aspect-w-3"), Style::Noop("hidden"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::OnScreen(Screen::Large, &[Style::Block])]);
						
						props.child("1", Dynamic).run("img", |mut props| {
							props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-02-secondary-product-shot.jpg");
							props.set_attribute("alt", "Two each of gray, white, and black shirts laying flat.");
							props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Large, &[Style::Noop("grid")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-y-8")])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-2"), Style::Noop("aspect-w-3"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-02-tertiary-product-shot-01.jpg");
								props.set_attribute("alt", "Model wearing plain black basic tee.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("aspect-h-2"), Style::Noop("aspect-w-3"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg")]);
							
							props.child("1", Dynamic).run("img", |mut props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-02-tertiary-product-shot-02.jpg");
								props.set_attribute("alt", "Model wearing plain gray basic tee.");
								props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
							});
						});
					});
					props.child("5", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("aspect-h-5"), Style::Noop("aspect-w-4"), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-h-4")]), Style::OnScreen(Screen::Large, &[Style::Noop("aspect-w-3")]), Style::OnScreen(Screen::Small, &[Style::Noop("overflow-hidden")]), Style::OnScreen(Screen::Small, &[Style::Noop("rounded-lg")])]);
						
						props.child("1", Dynamic).run("img", |mut props| {
							props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-02-featured-product-shot.jpg");
							props.set_attribute("alt", "Model wearing plain white basic tee.");
							props.styles(&[Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-center")]);
						});
					});
				});
				//  Product info 
				props.child("9", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::PaddingX(Size::Exact(16)), Style::PaddingBottom(Size::Exact(64)), Style::PaddingTop(Size::Exact(40)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::Noop("grid")]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-7xl")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-3")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-rows-[auto,auto,1fr]")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-8")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))]), Style::OnScreen(Screen::Large, &[Style::PaddingBottom(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::PaddingTop(Size::Exact(64))])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Large, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("border-r")]), Style::OnScreen(Screen::Large, &[Style::Noop("border-gray-200")]), Style::OnScreen(Screen::Large, &[Style::PaddingRight(Size::Exact(32))])]);
						
						props.child("1", Dynamic).run("h1", |mut props| {
							props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-3xl")])]);
							
							props.child("0", Label).run(|props| props.text("Basic Tee 6-Pack"));
						});
					});
					//  Options 
					props.child("5", Dynamic).run("div", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(16)), Style::OnScreen(Screen::Large, &[Style::Noop("row-span-3")]), Style::OnScreen(Screen::Large, &[Style::MarginTop(Size::Exact(0))])]);
						
						props.child("1", Dynamic).run("h2", |mut props| {
							props.styles(&[Style::Noop("sr-only")]);
							
							props.child("0", Label).run(|props| props.text("Product information"));
						});
						props.child("3", Dynamic).run("p", |mut props| {
							props.styles(&[Style::Noop("text-3xl"), Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("$192"));
						});
						//  Reviews 
						props.child("7", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(24))]);
							
							props.child("1", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.text("Reviews"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::TextColor(Color::Fg(100)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0")]);
									});
									props.child("3", Icon).run(|mut props| {
										props.style(&[Style::TextColor(Color::Fg(100)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0")]);
									});
									props.child("5", Icon).run(|mut props| {
										props.style(&[Style::TextColor(Color::Fg(100)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0")]);
									});
									props.child("7", Icon).run(|mut props| {
										props.style(&[Style::TextColor(Color::Fg(100)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0")]);
									});
									props.child("9", Icon).run(|mut props| {
										props.style(&[Style::TextColor(Color::Fg(22)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0")]);
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("4 out of 5 stars"));
								});
								props.child("5", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
									
									props.child("0", Label).run(|props| props.text("117 reviews"));
								});
							});
						});
						props.child("9", Dynamic).run("form", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(40))]);
							
							//  Colors 
							props.child("3", Dynamic).run("div", |mut props| {
								props.child("1", Dynamic).run("h3", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Color"));
								});
								props.child("3", Dynamic).run("fieldset", |mut props| {
									props.set_attribute("x-data", "window.Components.radioGroup({ initialCheckedIndex: 0 })");
									props.set_attribute("x-init", "init()");
									props.styles(&[Style::MarginTop(Size::Exact(16))]);
									
									props.child("1", Dynamic).run("legend", |mut props| {
										props.styles(&[Style::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.text("Choose a color"));
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Flex, Style::ItemsCenter, Style::SpaceX(Size::Exact(12))]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("relative"), Style::Noop("-m-0.5"), Style::Flex, Style::Noop("cursor-pointer"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-full"), Style::Padding(Size::Exact(2)), Style::OnFocus(&[Style::Noop("outline-none")]), Style::Noop("ring-gray-400")]);
											props.set_attribute("x-description", r#"Active and Checked: "ring ring-offset-1"	Not Active and Checked: "ring-2""#);
											props.set_attribute(":class", "{ 'ring ring-offset-1': (active === 'White') && (value === 'White'), 'undefined': !(active === 'White') || !(value === 'White'), 'ring-2': !(active === 'White') && (value === 'White'), 'undefined': (active === 'White') || !(value === 'White') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "color-choice");
												props.set_attribute("value", "White");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "color-choice-0-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "color-choice-0-label");
												props.styles(&[Style::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.text("White"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::Width(Size::Exact(32)), Style::Width(Size::Exact(32)), Style::Noop("bg-white"), Style::Noop("rounded-full"), Style::Noop("border"), Style::Noop("border-black"), Style::Noop("border-opacity-10")]);
											});
										});
										props.child("3", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("relative"), Style::Noop("-m-0.5"), Style::Flex, Style::Noop("cursor-pointer"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-full"), Style::Padding(Size::Exact(2)), Style::OnFocus(&[Style::Noop("outline-none")]), Style::Noop("ring-gray-400")]);
											props.set_attribute("x-description", r#"Active and Checked: "ring ring-offset-1"	Not Active and Checked: "ring-2""#);
											props.set_attribute(":class", "{ 'ring ring-offset-1': (active === 'Gray') && (value === 'Gray'), 'undefined': !(active === 'Gray') || !(value === 'Gray'), 'ring-2': !(active === 'Gray') && (value === 'Gray'), 'undefined': (active === 'Gray') || !(value === 'Gray') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "color-choice");
												props.set_attribute("value", "Gray");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "color-choice-1-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "color-choice-1-label");
												props.styles(&[Style::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.text("Gray"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::Width(Size::Exact(32)), Style::Width(Size::Exact(32)), Style::Color(Color::Fg(22)), Style::Noop("rounded-full"), Style::Noop("border"), Style::Noop("border-black"), Style::Noop("border-opacity-10")]);
											});
										});
										props.child("5", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("relative"), Style::Noop("-m-0.5"), Style::Flex, Style::Noop("cursor-pointer"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-full"), Style::Padding(Size::Exact(2)), Style::OnFocus(&[Style::Noop("outline-none")]), Style::Noop("ring-gray-900")]);
											props.set_attribute("x-description", r#"Active and Checked: "ring ring-offset-1"	Not Active and Checked: "ring-2""#);
											props.set_attribute(":class", "{ 'ring ring-offset-1': (active === 'Black') && (value === 'Black'), 'undefined': !(active === 'Black') || !(value === 'Black'), 'ring-2': !(active === 'Black') && (value === 'Black'), 'undefined': (active === 'Black') || !(value === 'Black') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "color-choice");
												props.set_attribute("value", "Black");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "color-choice-2-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "color-choice-2-label");
												props.styles(&[Style::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.text("Black"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::Width(Size::Exact(32)), Style::Width(Size::Exact(32)), Style::Color(Color::Fg(100)), Style::Noop("rounded-full"), Style::Noop("border"), Style::Noop("border-black"), Style::Noop("border-opacity-10")]);
											});
										});
									});
								});
							});
							//  Sizes 
							props.child("7", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(40))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
									
									props.child("1", Dynamic).run("h3", |mut props| {
										props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.text("Size"));
									});
									props.child("3", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
										
										props.child("0", Label).run(|props| props.text("Size guide"));
									});
								});
								props.child("3", Dynamic).run("fieldset", |mut props| {
									props.set_attribute("x-data", "window.Components.radioGroup({ initialCheckedIndex: 2 })");
									props.set_attribute("x-init", "init()");
									props.styles(&[Style::MarginTop(Size::Exact(16))]);
									
									props.child("1", Dynamic).run("legend", |mut props| {
										props.styles(&[Style::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.text("Choose a size"));
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-4"), Style::Noop("gap-4"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-8")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-4")])]);
										
										props.child("1", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(24))]), Style::Noop("cursor-not-allowed"), Style::Color(Color::Fg(6)), Style::TextColor(Color::Fg(22))]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XXS'), 'undefined': !(active === 'XXS') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "XXS");
												props.set_attribute("disabled", "");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-0-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "size-choice-0-label");
												
												props.child("0", Label).run(|props| props.text("XXS"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md"), Style::Noop("border-2"), Style::Noop("border-gray-200")]);
												
												props.child("1", Icon).run(|mut props| {
													props.style(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("stroke-2"), Style::TextColor(Color::Fg(22))]);
												});
											});
										});
										props.child("3", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(24))]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XS'), 'undefined': !(active === 'XS') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "XS");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-1-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "size-choice-1-label");
												
												props.child("0", Label).run(|props| props.text("XS"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'XS'), 'border-2': !(active === 'XS'), 'border-indigo-500': (value === 'XS'), 'border-transparent': !(value === 'XS') }");
											});
										});
										props.child("5", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(24))]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'S'), 'undefined': !(active === 'S') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "S");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-2-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "size-choice-2-label");
												
												props.child("0", Label).run(|props| props.text("S"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'S'), 'border-2': !(active === 'S'), 'border-indigo-500': (value === 'S'), 'border-transparent': !(value === 'S') }");
											});
										});
										props.child("7", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(24))]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'M'), 'undefined': !(active === 'M') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "M");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-3-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "size-choice-3-label");
												
												props.child("0", Label).run(|props| props.text("M"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'M'), 'border-2': !(active === 'M'), 'border-indigo-500': (value === 'M'), 'border-transparent': !(value === 'M') }");
											});
										});
										props.child("9", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(24))]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'L'), 'undefined': !(active === 'L') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "L");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-4-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "size-choice-4-label");
												
												props.child("0", Label).run(|props| props.text("L"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'L'), 'border-2': !(active === 'L'), 'border-indigo-500': (value === 'L'), 'border-transparent': !(value === 'L') }");
											});
										});
										props.child("11", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(24))]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XL'), 'undefined': !(active === 'XL') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "XL");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-5-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "size-choice-5-label");
												
												props.child("0", Label).run(|props| props.text("XL"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'XL'), 'border-2': !(active === 'XL'), 'border-indigo-500': (value === 'XL'), 'border-transparent': !(value === 'XL') }");
											});
										});
										props.child("13", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(24))]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === '2XL'), 'undefined': !(active === '2XL') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "2XL");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-6-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "size-choice-6-label");
												
												props.child("0", Label).run(|props| props.text("2XL"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === '2XL'), 'border-2': !(active === '2XL'), 'border-indigo-500': (value === '2XL'), 'border-transparent': !(value === '2XL') }");
											});
										});
										props.child("15", Dynamic).run("label", |mut props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(24))]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === '3XL'), 'undefined': !(active === '3XL') }");
											
											props.child("1", Dynamic).run("input", |mut props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "3XL");
												props.styles(&[Style::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-7-label");
											});
											props.child("3", Dynamic).run("span", |mut props| {
												props.set_attribute("id", "size-choice-7-label");
												
												props.child("0", Label).run(|props| props.text("3XL"));
											});
											props.child("5", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === '3XL'), 'border-2': !(active === '3XL'), 'border-indigo-500': (value === '3XL'), 'border-transparent': !(value === '3XL') }");
											});
										});
									});
								});
							});
							props.child("9", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "submit");
								props.styles(&[Style::MarginTop(Size::Exact(40)), Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-transparent"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(32)), Style::PaddingY(Size::Exact(12)), Style::Noop("text-base"), Style::Noop("font-medium"), Style::Noop("text-white"), Style::OnHover(&[Style::Noop("bg-indigo-700")]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-indigo-500")]), Style::OnFocus(&[Style::Noop("ring-offset-2")])]);
								
								props.child("0", Label).run(|props| props.text("Add to bag"));
							});
						});
					});
					props.child("7", Dynamic).run("div", |mut props| {
						props.styles(&[Style::PaddingY(Size::Exact(40)), Style::OnScreen(Screen::Large, &[Style::Noop("col-span-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("col-start-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("border-r")]), Style::OnScreen(Screen::Large, &[Style::Noop("border-gray-200")]), Style::OnScreen(Screen::Large, &[Style::PaddingBottom(Size::Exact(64))]), Style::OnScreen(Screen::Large, &[Style::PaddingRight(Size::Exact(32))]), Style::OnScreen(Screen::Large, &[Style::PaddingTop(Size::Exact(24))])]);
						
						//  Description and details 
						props.child("3", Dynamic).run("div", |mut props| {
							props.child("1", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.text("Description"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::SpaceY(Size::Exact(24))]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-base"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text(r#"The Basic Tee 6-Pack allows you to fully express your vibrant personality with three grayscale options. Feeling adventurous? Put on a heather gray tee. Want to be a trendsetter? Try our exclusive colorway: "Black". Need to add an extra pop of color to your outfit? Our white tee has you covered."#));
								});
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(40))]);
							
							props.child("1", Dynamic).run("h3", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Highlights"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(16))]);
								
								props.child("1", Dynamic).run("ul", |mut props| {
									props.set_attribute("role", "list");
									props.styles(&[Style::Noop("list-disc"), Style::SpaceY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(16)), Style::Noop("text-sm")]);
									
									props.child("1", Dynamic).run("li", |mut props| {
										props.styles(&[Style::TextColor(Color::Fg(44))]);
										
										props.child("0", Dynamic).run("span", |mut props| {
											props.styles(&[Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Hand cut and sewn locally"));
										});
									});
									props.child("3", Dynamic).run("li", |mut props| {
										props.styles(&[Style::TextColor(Color::Fg(44))]);
										
										props.child("0", Dynamic).run("span", |mut props| {
											props.styles(&[Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Dyed with our proprietary colors"));
										});
									});
									props.child("5", Dynamic).run("li", |mut props| {
										props.styles(&[Style::TextColor(Color::Fg(44))]);
										
										props.child("0", Dynamic).run("span", |mut props| {
											props.styles(&[Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Pre-washed & pre-shrunk"));
										});
									});
									props.child("7", Dynamic).run("li", |mut props| {
										props.styles(&[Style::TextColor(Color::Fg(44))]);
										
										props.child("0", Dynamic).run("span", |mut props| {
											props.styles(&[Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Ultra-soft 100% cotton"));
										});
									});
								});
							});
						});
						props.child("7", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(40))]);
							
							props.child("1", Dynamic).run("h2", |mut props| {
								props.styles(&[Style::Noop("text-sm"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Details"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(16)), Style::SpaceY(Size::Exact(24))]);
								
								props.child("1", Dynamic).run("p", |mut props| {
									props.styles(&[Style::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.text(r#"The 6-Pack includes two black, two white, and two heather gray Basic Tees. Sign up for our subscription service and be the first to get new, exciting colors, like our upcoming "Charcoal Gray" limited release."#));
								});
							});
						});
					});
				});
			});
		});
	}
}
