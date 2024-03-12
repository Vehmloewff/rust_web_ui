use rust_web_ui::*;

pub struct Example18;

pub struct Example18Props {}

impl Default for Example18Props {
	fn default() -> Example18Props {
		Example18Props { }
	}
}

impl Widget<'_> for Example18 {
	type Props = Example18Props;

	fn render(mut ctx: Ctx<'_>, props: Example18Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Flex, Style::Noop("min-h-[700px]")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("bg-white"), Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				props.set_attribute("style", "height: 700px");
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Width(Size::Exact(288))]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.set_attribute("x-data", "{ open: true }");
						props.set_attribute("@keydown.window.escape", "open = false");
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::SpaceX(Size::Exact(32)), Style::Noop("text-base"), Style::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("h3", |mut props| {
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										
										props.child("0", Label).run(|props| props.text("Basic Tee 6-Pack"));
									});
								});
								props.child("3", Dynamic).run("p", |mut props| {
									props.child("0", Label).run(|props| props.text("$192"));
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("aspect-h-3"), Style::Noop("aspect-w-4"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
								
								props.child("1", Dynamic).run("img", |mut props| {
									props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-quick-preview-02-preview.jpg");
									props.set_attribute("alt", "Model wearing gray t-shirt.");
									props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::Noop("items-end"), Style::Padding(Size::Exact(16))]);
									
									props.child("1", Dynamic).run("button", |mut props| {
										props.set_attribute("type", "button");
										props.styles(&[Style::Noop("relative"), Style::Noop("z-10"), Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::Noop("bg-opacity-75"), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(100)), Style::Noop("opacity-0"), Style::OnFocus(&[Style::Noop("opacity-100")]), Style::NoopGroup("group-hover", &[Style::Noop("opacity-100")])]);
										props.set_attribute("@click", "open = true");
										
										props.child("0", Label).run(|props| props.text("Quick View"));
										props.child("1", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("sr-only")]);
											
											props.child("0", Label).run(|props| props.text(", Basic Tee 6-Pack"));
										});
									});
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.set_attribute("x-show", "open");
							props.styles(&[Style::Noop("relative"), Style::Noop("z-10")]);
							props.set_attribute("x-ref", "dialog");
							props.set_attribute("aria-modal", "true");
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "ease-out duration-300");
								props.set_attribute("x-transition:enter-start", "opacity-0");
								props.set_attribute("x-transition:enter-end", "opacity-100");
								props.set_attribute("x-transition:leave", "ease-in duration-200");
								props.set_attribute("x-transition:leave-start", "opacity-100");
								props.set_attribute("x-transition:leave-end", "opacity-0");
								props.set_attribute("x-description", "Background backdrop, show/hide based on modal state.");
								props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("hidden"), Style::Color(Color::Fg(56)), Style::Noop("bg-opacity-75"), Style::Noop("transition-opacity"), Style::OnScreen(Screen::Medium, &[Style::Block])]);
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("z-10"), Style::Noop("w-screen"), Style::Noop("overflow-y-auto")]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::Noop("min-h-full"), Style::Noop("items-stretch"), Style::JustifyCenter, Style::Noop("text-center"), Style::OnScreen(Screen::Medium, &[Style::ItemsCenter]), Style::OnScreen(Screen::Medium, &[Style::PaddingX(Size::Exact(8))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(16))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.set_attribute("x-show", "open");
										props.set_attribute("x-transition:enter", "ease-out duration-300");
										props.set_attribute("x-transition:enter-start", "opacity-0 translate-y-4 md:translate-y-0 md:scale-95");
										props.set_attribute("x-transition:enter-end", "opacity-100 translate-y-0 md:scale-100");
										props.set_attribute("x-transition:leave", "ease-in duration-200");
										props.set_attribute("x-transition:leave-start", "opacity-100 translate-y-0 md:scale-100");
										props.set_attribute("x-transition:leave-end", "opacity-0 translate-y-4 md:translate-y-0 md:scale-95");
										props.set_attribute("x-description", "Modal panel, show/hide based on modal state.");
										props.styles(&[Style::Flex, Style::Width(Size::Full), Style::Noop("transform"), Style::Noop("text-left"), Style::Noop("text-base"), Style::Noop("transition"), Style::OnScreen(Screen::Medium, &[Style::MarginY(Size::Exact(32))]), Style::OnScreen(Screen::Medium, &[Style::Noop("max-w-2xl")]), Style::OnScreen(Screen::Medium, &[Style::PaddingX(Size::Exact(16))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-4xl")])]);
										props.set_attribute("@click.away", "open = false");
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("relative"), Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::Noop("overflow-hidden"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(16)), Style::PaddingBottom(Size::Exact(32)), Style::PaddingTop(Size::Exact(56)), Style::Noop("shadow-2xl"), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::PaddingTop(Size::Exact(32))]), Style::OnScreen(Screen::Medium, &[Style::Padding(Size::Exact(24))]), Style::OnScreen(Screen::Large, &[Style::Padding(Size::Exact(32))])]);
											
											props.child("1", Dynamic).run("button", |mut props| {
												props.set_attribute("type", "button");
												props.styles(&[Style::Noop("absolute"), Style::Noop("right-4"), Style::Noop("top-4"), Style::TextColor(Color::Fg(44)), Style::OnHover(&[Style::TextColor(Color::Fg(56))]), Style::OnScreen(Screen::Small, &[Style::Noop("right-6")]), Style::OnScreen(Screen::Small, &[Style::Noop("top-8")]), Style::OnScreen(Screen::Medium, &[Style::Noop("right-6")]), Style::OnScreen(Screen::Medium, &[Style::Noop("top-6")]), Style::OnScreen(Screen::Large, &[Style::Noop("right-8")]), Style::OnScreen(Screen::Large, &[Style::Noop("top-8")])]);
												props.set_attribute("@click", "open = false");
												
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("sr-only")]);
													
													props.child("0", Label).run(|props| props.text("Close"));
												});
												props.child("3", Icon).run(|mut props| {
													props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
												});
											});
											props.child("3", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("grid"), Style::Width(Size::Full), Style::Noop("grid-cols-1"), Style::Noop("items-start"), Style::Noop("gap-x-6"), Style::Noop("gap-y-8"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-12")]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-8")])]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(&[Style::Noop("aspect-h-3"), Style::Noop("aspect-w-2"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Style::OnScreen(Screen::Small, &[Style::Noop("col-span-4")]), Style::OnScreen(Screen::Large, &[Style::Noop("col-span-5")])]);
													
													props.child("1", Dynamic).run("img", |mut props| {
														props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-quick-preview-02-detail.jpg");
														props.set_attribute("alt", "Two each of gray, white, and black shirts arranged on table.");
														props.styles(&[Style::Noop("object-cover"), Style::Noop("object-center")]);
													});
												});
												props.child("3", Dynamic).run("div", |mut props| {
													props.styles(&[Style::OnScreen(Screen::Small, &[Style::Noop("col-span-8")]), Style::OnScreen(Screen::Large, &[Style::Noop("col-span-7")])]);
													
													props.child("1", Dynamic).run("h2", |mut props| {
														props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::PaddingRight(Size::Exact(48))])]);
														
														props.child("0", Label).run(|props| props.text("Basic Tee 6-Pack"));
													});
													props.child("3", Dynamic).run("section", |mut props| {
														props.set_attribute("aria-labelledby", "information-heading");
														props.styles(&[Style::MarginTop(Size::Exact(8))]);
														
														props.child("1", Dynamic).run("h3", |mut props| {
															props.set_attribute("id", "information-heading");
															props.styles(&[Style::Noop("sr-only")]);
															
															props.child("0", Label).run(|props| props.text("Product information"));
														});
														props.child("3", Dynamic).run("p", |mut props| {
															props.styles(&[Style::Noop("text-2xl"), Style::TextColor(Color::Fg(100))]);
															
															props.child("0", Label).run(|props| props.text("$192"));
														});
														//  Reviews 
														props.child("7", Dynamic).run("div", |mut props| {
															props.styles(&[Style::MarginTop(Size::Exact(24))]);
															
															props.child("1", Dynamic).run("h4", |mut props| {
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
																	
																	props.child("0", Label).run(|props| props.text("3.9 out of 5 stars"));
																});
																props.child("5", Dynamic).run("a", |mut props| {
																	props.set_attribute("href", "#");
																	props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
																	
																	props.child("0", Label).run(|props| props.text("117 reviews"));
																});
															});
														});
													});
													props.child("5", Dynamic).run("section", |mut props| {
														props.set_attribute("aria-labelledby", "options-heading");
														props.styles(&[Style::MarginTop(Size::Exact(40))]);
														
														props.child("1", Dynamic).run("h3", |mut props| {
															props.set_attribute("id", "options-heading");
															props.styles(&[Style::Noop("sr-only")]);
															
															props.child("0", Label).run(|props| props.text("Product options"));
														});
														props.child("3", Dynamic).run("form", |mut props| {
															//  Colors 
															props.child("3", Dynamic).run("div", |mut props| {
																props.child("1", Dynamic).run("h4", |mut props| {
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
																	props.child("3", Dynamic).run("span", |mut props| {
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
																	
																	props.child("1", Dynamic).run("h4", |mut props| {
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
																		props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-4"), Style::Noop("gap-4")]);
																		
																		props.child("1", Dynamic).run("label", |mut props| {
																			props.set_attribute("x-radio-group-option", "");
																			props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
																			props.set_attribute("x-state:on:size.instock", "In Stock");
																			props.set_attribute("x-state:off:size.instock", "Out of Stock");
																			props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
																			props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XXS'), 'undefined': !(active === 'XXS') }");
																			
																			props.child("1", Dynamic).run("input", |mut props| {
																				props.set_attribute("type", "radio");
																				props.set_attribute("x-model", "value");
																				props.set_attribute("name", "size-choice");
																				props.set_attribute("value", "XXS");
																				props.styles(&[Style::Noop("sr-only")]);
																				props.set_attribute("aria-labelledby", "size-choice-0-label");
																			});
																			props.child("3", Dynamic).run("span", |mut props| {
																				props.set_attribute("id", "size-choice-0-label");
																				
																				props.child("0", Label).run(|props| props.text("XXS"));
																			});
																			props.child("5", Dynamic).run("span", |mut props| {
																				props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
																				props.set_attribute("aria-hidden", "true");
																				props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
																				props.set_attribute(":class", "{ 'border': (active === 'XXS'), 'border-2': !(active === 'XXS'), 'border-indigo-500': (value === 'XXS'), 'border-transparent': !(value === 'XXS') }");
																			});
																		});
																		props.child("3", Dynamic).run("label", |mut props| {
																			props.set_attribute("x-radio-group-option", "");
																			props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
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
																			props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
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
																			props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
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
																			props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
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
																			props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
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
																			props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::Noop("cursor-pointer"), Style::Noop("bg-white"), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm")]);
																			props.set_attribute("x-state:on:size.instock", "In Stock");
																			props.set_attribute("x-state:off:size.instock", "Out of Stock");
																			props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
																			props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XXL'), 'undefined': !(active === 'XXL') }");
																			
																			props.child("1", Dynamic).run("input", |mut props| {
																				props.set_attribute("type", "radio");
																				props.set_attribute("x-model", "value");
																				props.set_attribute("name", "size-choice");
																				props.set_attribute("value", "XXL");
																				props.styles(&[Style::Noop("sr-only")]);
																				props.set_attribute("aria-labelledby", "size-choice-6-label");
																			});
																			props.child("3", Dynamic).run("span", |mut props| {
																				props.set_attribute("id", "size-choice-6-label");
																				
																				props.child("0", Label).run(|props| props.text("XXL"));
																			});
																			props.child("5", Dynamic).run("span", |mut props| {
																				props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md")]);
																				props.set_attribute("aria-hidden", "true");
																				props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
																				props.set_attribute(":class", "{ 'border': (active === 'XXL'), 'border-2': !(active === 'XXL'), 'border-indigo-500': (value === 'XXL'), 'border-transparent': !(value === 'XXL') }");
																			});
																		});
																		props.child("15", Dynamic).run("label", |mut props| {
																			props.set_attribute("x-radio-group-option", "");
																			props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::PaddingY(Size::Exact(12)), Style::PaddingX(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("uppercase"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-1")]), Style::Noop("cursor-not-allowed"), Style::Color(Color::Fg(6)), Style::TextColor(Color::Fg(22))]);
																			props.set_attribute("x-state:on:size.instock", "In Stock");
																			props.set_attribute("x-state:off:size.instock", "Out of Stock");
																			props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
																			props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XXXL'), 'undefined': !(active === 'XXXL') }");
																			
																			props.child("1", Dynamic).run("input", |mut props| {
																				props.set_attribute("type", "radio");
																				props.set_attribute("x-model", "value");
																				props.set_attribute("name", "size-choice");
																				props.set_attribute("value", "XXXL");
																				props.set_attribute("disabled", "");
																				props.styles(&[Style::Noop("sr-only")]);
																				props.set_attribute("aria-labelledby", "size-choice-7-label");
																			});
																			props.child("3", Dynamic).run("span", |mut props| {
																				props.set_attribute("id", "size-choice-7-label");
																				
																				props.child("0", Label).run(|props| props.text("XXXL"));
																			});
																			props.child("5", Dynamic).run("span", |mut props| {
																				props.set_attribute("aria-hidden", "true");
																				props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("-inset-px"), Style::Noop("rounded-md"), Style::Noop("border-2"), Style::Noop("border-gray-200")]);
																				
																				props.child("1", Icon).run(|mut props| {
																					props.style(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("stroke-2"), Style::TextColor(Color::Fg(22))]);
																				});
																			});
																		});
																	});
																});
															});
															props.child("9", Dynamic).run("button", |mut props| {
																props.set_attribute("type", "submit");
																props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("border"), Style::Noop("border-transparent"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(32)), Style::PaddingY(Size::Exact(12)), Style::Noop("text-base"), Style::Noop("font-medium"), Style::Noop("text-white"), Style::OnHover(&[Style::Noop("bg-indigo-700")]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-indigo-500")]), Style::OnFocus(&[Style::Noop("ring-offset-2")])]);
																
																props.child("0", Label).run(|props| props.text("Add to bag"));
															});
														});
													});
												});
											});
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
